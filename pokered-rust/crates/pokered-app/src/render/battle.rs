use pokered_core::battle::{BattlePhase, BattleScreen};
use pokered_renderer::battle_scene::{EnemyHud, PlayerHud};
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::{AssetCategory, LoadedPng, ResourceManager};
use pokered_renderer::text_renderer::{write_tiles_at, ScreenTileBuffer};
use pokered_renderer::textbox::TextBoxFrame;
use pokered_renderer::tile::TileSet;
use pokered_renderer::{FrameBuffer, Rgba, TILE_SIZE};

use super::{blit_tileset, species_to_sprite_name};

// ---------------------------------------------------------------------------
// ASCII → Pokémon charmap conversion
// ---------------------------------------------------------------------------

/// Convert an ASCII string to a vector of Pokémon Red tile IDs.
///
/// Matches the charmap in constants/charmap.asm:
///   'A'-'Z' → $80-$99, 'a'-'z' → $A0-$B9,
///   '0'-'9' → $F6-$FF, ' ' → $7F, ':' → $9C, '/' → $F3, etc.
fn ascii_to_tiles(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            'A'..='Z' => 0x80 + (c as u8 - b'A'),
            'a'..='z' => 0xA0 + (c as u8 - b'a'),
            '0'..='9' => 0xF6 + (c as u8 - b'0'),
            ' ' => 0x7F,
            ':' => 0x9C,
            '/' => 0xF3,
            '(' => 0x9A,
            ')' => 0x9B,
            '-' => 0xE3,
            '.' => 0xE8,
            '\'' => 0xE0,
            '!' => 0xE7,
            '?' => 0xE6,
            '>' => 0xED, // used as cursor arrow
            _ => 0x7F,   // space for unknown
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Combined VRAM tileset construction
// ---------------------------------------------------------------------------

/// Build the combined 256-tile VRAM tileset that mirrors the Game Boy's
/// VRAM layout during battle.
///
/// Tile ID mapping (from home/load_font.asm):
///   $80-$FF: font.png (1bpp, 128 tiles) — A-Z, a-z, digits, punctuation
///   $60-$7F: font_extra.png (2bpp, 32 tiles) — textbox borders, then
///   $62-$7F: font_battle_extra.png (2bpp, 30 tiles) — HP bar tiles (OVERWRITES $62+)
///   $6D+: battle_hud_1.png (2bpp, 3 tiles) — end cap, Lv, triangle
///   $73+: battle_hud_2.png + battle_hud_3.png (2bpp, 3+3=6 tiles) — HUD borders
fn build_battle_tileset(rm: &mut ResourceManager) -> TileSet {
    let mut ts = TileSet::blank(256);

    // 1. Font tiles at $80-$FF (128 tiles from font.png, loaded as 1bpp)
    if let Ok(cached) = rm.load_font("font") {
        let font_ts = cached.tileset.clone();
        for i in 0..font_ts.len().min(128) {
            ts.set(0x80 + i, font_ts.get(i).clone());
        }
    }

    // 2. TextBox tiles at $60-$7F (from font_extra.png, 2bpp)
    //    Must load as 2bpp — can't use load_font() which forces 1bpp.
    if let Ok(path) = rm
        .root()
        .resolve_checked(AssetCategory::Font, "font_extra.png")
    {
        if let Ok(loaded) = LoadedPng::load(&path) {
            if let Ok(extra_ts) = loaded.to_tileset(false) {
                // false = 2bpp
                for i in 0..extra_ts.len().min(32) {
                    ts.set(0x60 + i, extra_ts.get(i).clone());
                }
            }
        }
    }

    // 3. HP bar + status tiles at $62+ (from font_battle_extra.png, 2bpp)
    //    OVERWRITES $62+ from step 2.
    if let Ok(path) = rm
        .root()
        .resolve_checked(AssetCategory::Font, "font_battle_extra.png")
    {
        if let Ok(loaded) = LoadedPng::load(&path) {
            if let Ok(hp_ts) = loaded.to_tileset(false) {
                for i in 0..hp_ts.len() {
                    ts.set(0x62 + i, hp_ts.get(i).clone());
                }
            }
        }
    }

    // 4. Battle HUD tiles
    //    battle_hud_1.png (2bpp, 3 tiles) → $6D
    if let Ok(cached) = rm.load_battle("battle_hud_1") {
        let hud1 = cached.tileset.clone();
        for i in 0..hud1.len() {
            ts.set(0x6D + i, hud1.get(i).clone());
        }
    }

    //    battle_hud_2.png (2bpp, 3 tiles) → $73
    //    battle_hud_3.png (2bpp, 3 tiles) → concatenated after hud_2 at $73+3
    if let Ok(cached) = rm.load_battle("battle_hud_2") {
        let hud2 = cached.tileset.clone();
        let hud2_len = hud2.len();
        for i in 0..hud2_len {
            ts.set(0x73 + i, hud2.get(i).clone());
        }
        if let Ok(cached3) = rm.load_battle("battle_hud_3") {
            let hud3 = cached3.tileset.clone();
            for i in 0..hud3.len() {
                ts.set(0x73 + hud2_len + i, hud3.get(i).clone());
            }
        }
    }

    ts
}

// ---------------------------------------------------------------------------
// Battle menu text (tile-encoded)
// ---------------------------------------------------------------------------

/// Draw the 2×2 battle menu items into the tile buffer.
///
/// Original layout (from DisplayBattleMenu in engine/battle/core.asm):
///   The battle menu is in the right half of the bottom text box.
///   Row 14: "FIGHT" at (10,14), "PkMn" at (15,14) — but we use charmap tiles
///   Row 16: "ITEM" at (10,16), "RUN" at (15,16)
fn draw_battle_menu(buf: &mut ScreenTileBuffer, selected_row: usize, selected_col: usize) {
    // Battle menu inner box border (right half of dialog area)
    // From DrawPlayerBattleMenu: a 2-column wide inner box at (8,12) 12×6
    // We draw a sub-box on the right side
    let menu_box = TextBoxFrame::new(8, 12, 12, 6);
    menu_box.draw_frame(buf);

    let fight_tiles = ascii_to_tiles("FIGHT");
    let pkmn_tiles = ascii_to_tiles("PKMN");
    let item_tiles = ascii_to_tiles("ITEM");
    let run_tiles = ascii_to_tiles("RUN");

    write_tiles_at(buf, 10, 14, &fight_tiles);
    write_tiles_at(buf, 15, 14, &pkmn_tiles);
    write_tiles_at(buf, 10, 16, &item_tiles);
    write_tiles_at(buf, 15, 16, &run_tiles);

    // Selection cursor (▶ = $ED in charmap)
    let cursor_positions: [(u32, u32); 4] = [(9, 14), (14, 14), (9, 16), (14, 16)];
    let sel = selected_row * 2 + selected_col;
    if sel < cursor_positions.len() {
        let (cx, cy) = cursor_positions[sel];
        buf.set(cx, cy, 0xED); // ▶ tile
    }
}

/// Draw battle dialog text into the text box area.
fn draw_battle_text(buf: &mut ScreenTileBuffer, text: &str) {
    let tiles = ascii_to_tiles(text);
    // Text starts at (1, 14) — inside the dialog box, first text line
    write_tiles_at(buf, 1, 14, &tiles);
}

// ---------------------------------------------------------------------------
// Main battle rendering
// ---------------------------------------------------------------------------

pub fn draw_battle(screen: &BattleScreen, res: &mut Option<ResourceManager>, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    let enemy_name = format!("{}", screen.enemy_species).to_uppercase();
    let player_name = format!("{}", screen.player_species).to_uppercase();
    let enemy_sprite = species_to_sprite_name(&format!("{}", screen.enemy_species));
    let player_sprite = species_to_sprite_name(&format!("{}", screen.player_species));

    // Build combined VRAM tileset and tile buffer
    let mut tile_buf = ScreenTileBuffer::new(); // filled with $7F (space)

    if let Some(ref mut rm) = res {
        // ── Build combined 256-tile VRAM tileset ─────────────────────
        let battle_ts = build_battle_tileset(rm);

        // ── Enemy HUD (top-left) ─────────────────────────────────────
        let enemy_name_tiles = ascii_to_tiles(&enemy_name);
        EnemyHud::draw(
            &mut tile_buf,
            &enemy_name_tiles,
            screen.enemy_level,
            None, // no status ailment display for now
            screen.enemy_hp,
            screen.enemy_max_hp,
        );

        // ── Player HUD (right side) ─────────────────────────────────
        let player_name_tiles = ascii_to_tiles(&player_name);
        PlayerHud::draw(
            &mut tile_buf,
            &player_name_tiles,
            screen.player_level,
            None, // no status ailment display for now
            screen.player_hp,
            screen.player_max_hp,
        );

        // ── Bottom area (text box + menu or message) ─────────────────
        // Standard dialog box: full width, bottom 6 rows
        let dialog_box = TextBoxFrame::standard_dialog();
        dialog_box.draw_frame(&mut tile_buf);

        if matches!(screen.phase, BattlePhase::PlayerMenu) {
            // Battle menu in right half
            draw_battle_menu(
                &mut tile_buf,
                screen.battle_menu.row(),
                screen.battle_menu.col(),
            );
        } else {
            // Phase-dependent message text
            let phase_text = match &screen.phase {
                BattlePhase::Intro { .. } => {
                    if screen.is_wild {
                        format!("Wild {} appeared!", enemy_name)
                    } else {
                        format!("{} wants to fight!", enemy_name)
                    }
                }
                BattlePhase::TurnExecution { .. } => format!("{} used SCRATCH!", player_name),
                BattlePhase::FaintCheck { .. } => format!("Enemy {} fainted!", enemy_name),
                BattlePhase::Finished { won, .. } => {
                    if *won {
                        "You won!".to_string()
                    } else {
                        "You lost...".to_string()
                    }
                }
                _ => String::new(),
            };
            if !phase_text.is_empty() {
                draw_battle_text(&mut tile_buf, &phase_text);
            }
        }

        // ── Render tile buffer to framebuffer ────────────────────────
        tile_buf.render(fb, &battle_ts, pal);

        // ── Overlay Pokémon sprites on top ───────────────────────────
        // Enemy front sprite: 7×7 tiles at tile (12, 0) = pixel (96, 0)
        if let Ok(cached) = rm.load_pokemon_front(&enemy_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 12 * TILE_SIZE, 0, tpr, pal);
        }

        // Player back sprite: at tile (1, 5) = pixel (8, 40)
        if let Ok(cached) = rm.load_pokemon_back(&player_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 1 * TILE_SIZE, 5 * TILE_SIZE, tpr, pal);
        }
    } else {
        // No resources — fallback: render tile buffer with blank tileset
        let blank_ts = TileSet::blank(256);
        tile_buf.render(fb, &blank_ts, pal);
    }
}
