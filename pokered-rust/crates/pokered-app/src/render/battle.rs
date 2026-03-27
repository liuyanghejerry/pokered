use pokered_core::battle::{BattlePhase, BattleScreen};
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::ResourceManager;
use pokered_renderer::{FrameBuffer, Rgba, TILE_SIZE};

use super::{blit_tileset, draw_hp_bar, draw_text_box, species_to_sprite_name};

pub fn draw_battle(screen: &BattleScreen, res: &mut Option<ResourceManager>, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    let enemy_name = format!("{}", screen.enemy_species).to_uppercase();
    let player_name = format!("{}", screen.player_species).to_uppercase();
    let enemy_sprite = species_to_sprite_name(&format!("{}", screen.enemy_species));
    let player_sprite = species_to_sprite_name(&format!("{}", screen.player_species));

    if let Some(ref mut rm) = res {
        if let Ok(cached) = rm.load_pokemon_front(&enemy_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 12 * TILE_SIZE, 0, tpr, pal);
        }

        if let Ok(cached) = rm.load_pokemon_back(&player_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 1 * TILE_SIZE, 5 * TILE_SIZE, tpr, pal);
        }
    }

    draw_text_box(fb, 0, 0, 12, 3, Rgba::BLACK);
    draw_text(&enemy_name, 1 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    let enemy_lv = format!("Lv{}", screen.enemy_level);
    draw_text(&enemy_lv, 8 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text("HP:", 1 * TILE_SIZE, 2 * TILE_SIZE, Rgba::BLACK, fb);
    draw_hp_bar(
        fb,
        4 * TILE_SIZE,
        2 * TILE_SIZE + 2,
        8 * TILE_SIZE,
        screen.enemy_hp,
        screen.enemy_max_hp,
    );

    draw_text_box(fb, 9 * TILE_SIZE, 7 * TILE_SIZE, 10, 4, Rgba::BLACK);
    draw_text(&player_name, 10 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    let player_lv = format!("Lv{}", screen.player_level);
    draw_text(&player_lv, 17 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text("HP:", 10 * TILE_SIZE, 9 * TILE_SIZE, Rgba::BLACK, fb);
    draw_hp_bar(
        fb,
        13 * TILE_SIZE,
        9 * TILE_SIZE + 2,
        6 * TILE_SIZE,
        screen.player_hp,
        screen.player_max_hp,
    );
    let hp_text = format!("{:>3}/{:>3}", screen.player_hp, screen.player_max_hp);
    draw_text(&hp_text, 12 * TILE_SIZE, 10 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text_box(fb, 8 * TILE_SIZE, 12 * TILE_SIZE, 10, 4, Rgba::BLACK);

    if matches!(screen.phase, BattlePhase::PlayerMenu) {
        draw_text("FIGHT", 10 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("PkMn", 16 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("ITEM", 10 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("RUN", 16 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);

        let menu_labels_pos = [(10, 14), (16, 14), (10, 16), (16, 16)];
        let sel = screen.battle_menu.row() * 2 + screen.battle_menu.col();
        if sel < menu_labels_pos.len() {
            let (cx, cy) = menu_labels_pos[sel];
            draw_text(">", (cx - 1) * TILE_SIZE, cy * TILE_SIZE, Rgba::BLACK, fb);
        }
    } else {
        draw_text_box(fb, 0, 12 * TILE_SIZE, 18, 4, Rgba::BLACK);
        let phase_text = match &screen.phase {
            BattlePhase::Intro { .. } => format!("Wild {} appeared!", enemy_name),
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
            draw_text(&phase_text, 1 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        }
    }
}
