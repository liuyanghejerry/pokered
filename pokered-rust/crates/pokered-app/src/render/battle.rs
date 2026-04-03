use pokered_core::battle::state::StatusCondition as CoreStatus;
use pokered_core::battle::{BattlePhase, BattleScreen};
use pokered_renderer::battle_scene::{
    EnemyHud, PlayerHud, PokeballIndicators, PokeballStatus, StatusCondition,
};
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::{AssetCategory, LoadedPng, ResourceManager};
use pokered_renderer::text_renderer::{write_tiles_at, ScreenTileBuffer};
use pokered_renderer::textbox::TextBoxFrame;
use pokered_renderer::tile::{Tile, TileSet, TILE_PIXELS};
use pokered_renderer::{FrameBuffer, Rgba, TILE_SIZE};

use super::{blit_tileset, species_to_sprite_name};

// ---------------------------------------------------------------------------
// ScaleSpriteByTwo — faithful port of engine/battle/scale_sprites.asm
// ---------------------------------------------------------------------------

/// Scale a 4×4-tile (32×32 px) sprite to 7×7 tiles (56×56 px).
///
/// Matches the original `ScaleSpriteByTwo` algorithm:
///   1. Take only the top-left 28×28 pixels (ignore last 4 rows & cols).
///   2. Double every pixel in both X and Y → 56×56 pixels.
///   3. Pack the result into 7×7 = 49 tiles.
fn scale_sprite_by_two(src: &TileSet, src_tpr: usize) -> TileSet {
    const SRC_USED: usize = 28; // 32 - 4 = 28 pixels used per axis
    const DST_SIZE: usize = 56; // 28 * 2 = 56 pixels output per axis
    const DST_TILES: usize = 7; // 56 / 8 = 7 tiles per axis

    // 1. Extract 28×28 pixel grid from the source tileset
    let mut src_px = [[0u8; SRC_USED]; SRC_USED];
    for py in 0..SRC_USED {
        for px in 0..SRC_USED {
            let tile_col = px / TILE_PIXELS;
            let tile_row = py / TILE_PIXELS;
            let tile_idx = tile_row * src_tpr + tile_col;
            let local_col = px % TILE_PIXELS;
            let local_row = py % TILE_PIXELS;
            src_px[py][px] = src.get(tile_idx).pixels[local_row][local_col];
        }
    }

    // 2. Double each pixel in both X and Y → 56×56
    let mut dst_px = [[0u8; DST_SIZE]; DST_SIZE];
    for sy in 0..SRC_USED {
        for sx in 0..SRC_USED {
            let c = src_px[sy][sx];
            let dx = sx * 2;
            let dy = sy * 2;
            dst_px[dy][dx] = c;
            dst_px[dy][dx + 1] = c;
            dst_px[dy + 1][dx] = c;
            dst_px[dy + 1][dx + 1] = c;
        }
    }

    // 3. Pack 56×56 pixel grid into 7×7 tiles
    let mut out = TileSet::blank(DST_TILES * DST_TILES);
    for ty in 0..DST_TILES {
        for tx in 0..DST_TILES {
            let mut pixels = [[0u8; TILE_PIXELS]; TILE_PIXELS];
            for row in 0..TILE_PIXELS {
                for col in 0..TILE_PIXELS {
                    pixels[row][col] = dst_px[ty * TILE_PIXELS + row][tx * TILE_PIXELS + col];
                }
            }
            out.set(ty * DST_TILES + tx, Tile { pixels });
        }
    }

    out
}

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

fn build_party_pokeballs(party_size: usize) -> [PokeballStatus; 6] {
    let mut balls = [PokeballStatus::Empty; 6];
    for i in 0..party_size.min(6) {
        balls[i] = PokeballStatus::Normal;
    }
    balls
}

fn core_status_to_tiles(status: &CoreStatus) -> Option<StatusCondition> {
    match status {
        CoreStatus::None => None,
        CoreStatus::Sleep(_) => Some(StatusCondition::Sleep),
        CoreStatus::Poison => Some(StatusCondition::Poison),
        CoreStatus::Burn => Some(StatusCondition::Burn),
        CoreStatus::Freeze => Some(StatusCondition::Freeze),
        CoreStatus::Paralysis => Some(StatusCondition::Paralysis),
    }
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
///   $6D+: battle_hud_1.png (1bpp, 3 tiles) — end cap, Lv, triangle
///   $73+: battle_hud_2.png + battle_hud_3.png (1bpp, 3+3=6 tiles) — HUD borders
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

    // 4. Battle HUD tiles — loaded as **1bpp** (matching ASM's FarCopyDataDouble)
    //    The PNGs are 2-bit grayscale but the original game INCBINs them as .1bpp
    //    and loads via CopyVideoDataDouble which doubles each byte (1bpp→2bpp).
    //    battle_hud_1.png (1bpp, 3 tiles) → $6D
    if let Ok(path) = rm
        .root()
        .resolve_checked(AssetCategory::Battle, "battle_hud_1.png")
    {
        if let Ok(loaded) = LoadedPng::load(&path) {
            if let Ok(hud1) = loaded.to_tileset(true) {
                for i in 0..hud1.len() {
                    ts.set(0x6D + i, hud1.get(i).clone());
                }
            }
        }
    }

    //    battle_hud_2.png (1bpp, 3 tiles) → $73
    //    battle_hud_3.png (1bpp, 3 tiles) → concatenated after hud_2 at $73+3
    if let Ok(path) = rm
        .root()
        .resolve_checked(AssetCategory::Battle, "battle_hud_2.png")
    {
        if let Ok(loaded) = LoadedPng::load(&path) {
            if let Ok(hud2) = loaded.to_tileset(true) {
                let hud2_len = hud2.len();
                for i in 0..hud2_len {
                    ts.set(0x73 + i, hud2.get(i).clone());
                }
                if let Ok(path3) = rm
                    .root()
                    .resolve_checked(AssetCategory::Battle, "battle_hud_3.png")
                {
                    if let Ok(loaded3) = LoadedPng::load(&path3) {
                        if let Ok(hud3) = loaded3.to_tileset(true) {
                            for i in 0..hud3.len() {
                                ts.set(0x73 + hud2_len + i, hud3.get(i).clone());
                            }
                        }
                    }
                }
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
    let pkmn_tiles: Vec<u8> = vec![0xE1, 0xE2]; // <PK><MN> charmap tiles
    let item_tiles = ascii_to_tiles("ITEM");
    let run_tiles = ascii_to_tiles("RUN");

    write_tiles_at(buf, 10, 14, &fight_tiles);
    write_tiles_at(buf, 16, 14, &pkmn_tiles);
    write_tiles_at(buf, 10, 16, &item_tiles);
    write_tiles_at(buf, 16, 16, &run_tiles);

    // Selection cursor (▶ = $ED in charmap)
    // row=0/1 (top/bottom), col=0/1 (left/right)
    let cursor_x = if selected_col == 0 { 9 } else { 15 };
    let cursor_y = if selected_row == 0 { 14 } else { 16 };
    buf.set(cursor_x, cursor_y, 0xED);
}

/// Draw battle dialog text into the text box area.
fn draw_battle_text(buf: &mut ScreenTileBuffer, text: &str) {
    const LINE_WIDTH: usize = 18;

    let mut wrapped: Vec<String> = Vec::new();

    for raw_line in text.split('\n') {
        let words: Vec<&str> = raw_line.split_whitespace().collect();
        if words.is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let mut current = String::new();
        for word in words {
            let word_chars: Vec<char> = word.chars().collect();
            let mut start = 0;
            while start < word_chars.len() {
                let end = (start + LINE_WIDTH).min(word_chars.len());
                let part: String = word_chars[start..end].iter().collect();

                if current.is_empty() {
                    current.push_str(&part);
                } else if current.chars().count() + 1 + part.chars().count() <= LINE_WIDTH {
                    current.push(' ');
                    current.push_str(&part);
                } else {
                    wrapped.push(current);
                    current = part;
                }

                start = end;
            }
        }

        if !current.is_empty() {
            wrapped.push(current);
        }
    }

    if let Some(line1) = wrapped.first() {
        write_tiles_at(buf, 1, 14, &ascii_to_tiles(line1));
    }
    if let Some(line2) = wrapped.get(1) {
        write_tiles_at(buf, 1, 16, &ascii_to_tiles(line2));
    }
}

fn move_display_name(move_id: pokered_data::moves::MoveId) -> String {
    let raw = format!("{:?}", move_id);
    let mut result = String::with_capacity(raw.len() + 4);
    for (i, c) in raw.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev = raw.as_bytes()[i - 1] as char;
            if prev.is_lowercase() {
                result.push(' ');
            }
        }
        result.push(c);
    }
    result.to_uppercase()
}

fn draw_move_menu(buf: &mut ScreenTileBuffer, screen: &BattleScreen) {
    if let Some(ref mm) = screen.move_menu {
        // Original: TextBoxBorder(4, 12, 14, 4), moves at hlcoord(6, 13), cursor at col 5
        let move_box = TextBoxFrame::new(4, 12, 16, 6);
        move_box.draw_frame(buf);

        // Match engine/battle/core.asm MoveSelectionMenu:
        // after drawing the move box, top border is patched at (4,12)='─' and (10,12)='┘'
        // to join the left TYPE/PP panel cleanly.
        buf.set(4, 12, 0x7A);
        buf.set(10, 12, 0x7E);

        let moves = mm.moves();
        for (i, slot) in moves.iter().enumerate() {
            let name = move_display_name(slot.move_id);
            let truncated: String = name.chars().take(12).collect();
            let name_tiles = ascii_to_tiles(&truncated);
            let y = 13 + i as u32;
            write_tiles_at(buf, 6, y, &name_tiles);
        }

        let cursor_y = 13 + mm.cursor() as u32;
        buf.set(5, cursor_y, 0xED);

        // Original: TextBoxBorder(0, 8, 3, 9) — TYPE/PP info for highlighted move
        let pp_box = TextBoxFrame::new(0, 8, 11, 5);
        pp_box.draw_frame(buf);

        let cursor_idx = mm.cursor();
        if cursor_idx < moves.len() {
            let slot = &moves[cursor_idx];
            let type_label = ascii_to_tiles("TYPE/");
            write_tiles_at(buf, 1, 9, &type_label);

            if let Some(move_data) = pokered_data::move_data::MoveData::get(slot.move_id) {
                let type_name = format!("{:?}", move_data.move_type).to_uppercase();
                let type_tiles = ascii_to_tiles(&type_name);
                write_tiles_at(buf, 1, 10, &type_tiles);
            }

            // Match PrintMenuItem in engine/battle/core.asm:
            // (5,9)='/', (7,11)='/', current PP at (5,11), max PP at (8,11), plus "PP" label.
            let pp_label = ascii_to_tiles("PP");
            write_tiles_at(buf, 2, 11, &pp_label);

            let pp_text = format!("{:>2}/{:>2}", slot.current_pp.min(99), slot.max_pp.min(99));
            let pp_tiles = ascii_to_tiles(&pp_text);
            write_tiles_at(buf, 5, 11, &pp_tiles);
        }
    }

    if let Some(ref msg) = screen.current_message {
        let tiles = ascii_to_tiles(msg);
        write_tiles_at(buf, 1, 14, &tiles);
    }
}

fn draw_party_menu(buf: &mut ScreenTileBuffer, screen: &BattleScreen) {
    if let Some(ref bs) = screen.battle_state {
        for (i, mon) in bs.player.party.iter().enumerate() {
            let name = format!("{}", mon.species).to_uppercase();
            let line = if mon.hp == 0 {
                format!("{} FNT", name)
            } else {
                format!("{} {}/{}", name, mon.hp, mon.max_hp)
            };
            let tiles = ascii_to_tiles(&line);
            let y = 14 + (i.min(3)) as u32;
            write_tiles_at(buf, 2, y, &tiles);
        }
        let cursor_y = 14 + (screen.party_cursor.min(3)) as u32;
        buf.set(1, cursor_y, 0xED);
    }

    if let Some(ref msg) = screen.current_message {
        let msg_tiles = ascii_to_tiles(msg);
        write_tiles_at(buf, 1, 16, &msg_tiles);
    }
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
        let enemy_status_tiles = core_status_to_tiles(&screen.enemy_status).map(|s| s.tiles());
        let enemy_hp_color = EnemyHud::draw(
            &mut tile_buf,
            &enemy_name_tiles,
            screen.enemy_level,
            enemy_status_tiles.as_ref().map(|t| t.as_slice()),
            screen.enemy_hp,
            screen.enemy_max_hp,
        );

        // ── Player HUD (right side) ─────────────────────────────────
        let player_name_tiles = ascii_to_tiles(&player_name);
        let player_status_tiles = core_status_to_tiles(&screen.player_status).map(|s| s.tiles());
        let player_hp_color = PlayerHud::draw(
            &mut tile_buf,
            &player_name_tiles,
            screen.player_level,
            player_status_tiles.as_ref().map(|t| t.as_slice()),
            screen.player_hp,
            screen.player_max_hp,
        );
        // Pokeball party indicators
        let player_balls = build_party_pokeballs(screen.player_party_size);
        let enemy_balls = build_party_pokeballs(screen.enemy_party_size);
        PokeballIndicators::draw_player(&mut tile_buf, &player_balls);
        PokeballIndicators::draw_enemy(&mut tile_buf, &enemy_balls);

        // ── Bottom area (text box + menu or message) ─────────────────
        // Standard dialog box: full width, bottom 6 rows
        let dialog_box = TextBoxFrame::standard_dialog();
        dialog_box.draw_frame(&mut tile_buf);

        if matches!(screen.phase, BattlePhase::PlayerMenu) {
            draw_battle_menu(
                &mut tile_buf,
                screen.battle_menu.row(),
                screen.battle_menu.col(),
            );
        } else if matches!(screen.phase, BattlePhase::MoveSelect) {
            draw_move_menu(&mut tile_buf, screen);
        } else if matches!(
            screen.phase,
            BattlePhase::PartySelect | BattlePhase::PlayerFaintSwitch
        ) {
            draw_party_menu(&mut tile_buf, screen);
        } else {
            let phase_text = match &screen.phase {
                BattlePhase::Intro { .. } => {
                    if screen.is_wild {
                        Some(format!("Wild {} appeared!", enemy_name))
                    } else {
                        Some(format!("{} wants to fight!", enemy_name))
                    }
                }
                BattlePhase::BattleOver { won, .. } => {
                    if *won {
                        Some("You won!".to_string())
                    } else {
                        Some("You lost...".to_string())
                    }
                }
                _ => screen.current_message.clone(),
            };
            if let Some(ref text) = phase_text {
                draw_battle_text(&mut tile_buf, text);
            }
        }

        // ── Render tile buffer to framebuffer ────────────────────────
        tile_buf.render(fb, &battle_ts, pal);

        // ── Apply HP bar color palettes ──────────────────────────────
        // SGB BlkPacket_Battle defines palette regions:
        //   Enemy:  pal 1, rect (1,0)-(10,3)  — name/level + HP bar
        //   Player: pal 0, rect (10,7)-(19,10) — name/level + HP bar + HP numbers
        // We apply colored HP palettes only to the HP bar rows (+ HP numbers for player),
        // since coloring name/level text with green/yellow/red would look wrong.
        // Enemy HP bar: tiles (2,2) through (10,2) — 9 tiles wide, 1 row
        tile_buf.render_region(fb, &battle_ts, enemy_hp_color.palette(), 2, 2, 9, 1);
        // Player HP bar + HP numbers: tiles (10,9) through (18,10) — 9 tiles wide, 2 rows
        tile_buf.render_region(fb, &battle_ts, player_hp_color.palette(), 10, 9, 9, 2);

        // ── Overlay Pokémon sprites on top ───────────────────────────
        // Enemy front sprite: centered within 7×7 tile area at tile (12, 0)
        // Centering matches LoadUncompressedSpriteData in home/pics.asm:
        //   x_offset = ((8 - w_tiles) / 2) * 8px,  y_offset = (7 - h_tiles) * 8px
        if let Ok(cached) = rm.load_pokemon_front(&enemy_sprite) {
            let ts = cached.tileset.clone();
            let w_tiles = cached.source_size.0 / TILE_SIZE;
            let h_tiles = cached.source_size.1 / TILE_SIZE;
            let x_off = ((8 - w_tiles) / 2) * TILE_SIZE;
            let y_off = (7 - h_tiles) * TILE_SIZE;
            blit_tileset(fb, &ts, 12 * TILE_SIZE + x_off, y_off, w_tiles, pal);
        }

        // Player back sprite: loaded as 4×4 tiles (32×32), scaled to 7×7 (56×56)
        // via ScaleSpriteByTwo, then blitted at tile (1, 5) = pixel (8, 40)
        let back_sprite_name = format!("{}b", player_sprite);
        if let Ok(cached) = rm.load_pokemon_back(&back_sprite_name) {
            let ts = cached.tileset.clone();
            let src_tpr = (cached.source_size.0 / TILE_SIZE) as usize;
            let scaled = scale_sprite_by_two(&ts, src_tpr);
            blit_tileset(fb, &scaled, 1 * TILE_SIZE, 5 * TILE_SIZE, 7, pal);
        }

        // In Gen1 move-select, the TYPE/PP panel overlays the player sprite.
        // Our sprite blit happens after tilemap rendering, so redraw this panel
        // region last to keep it in the foreground.
        if matches!(screen.phase, BattlePhase::MoveSelect) {
            tile_buf.render_region(fb, &battle_ts, pal, 0, 8, 11, 5);
        }
    } else {
        // No resources — fallback: render tile buffer with blank tileset
        let blank_ts = TileSet::blank(256);
        tile_buf.render(fb, &blank_ts, pal);
    }
}
