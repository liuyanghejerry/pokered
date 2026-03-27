use pokered_core::oak_speech::{
    OakSpeechPhase, OakSpeechState, DEFAULT_PLAYER_NAMES, DEFAULT_RIVAL_NAMES,
};
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::{AssetCategory, ResourceManager};
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_WIDTH, TILE_SIZE};

use super::{draw_centered_sprite, draw_text_box};

const TEXT_BOX_X: u32 = 0;
const TEXT_BOX_Y: u32 = 12 * 8;
const TEXT_BOX_W: u32 = 18;
const TEXT_BOX_H: u32 = 4;

pub fn draw_oak_speech(
    state: &OakSpeechState,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    if let Some(naming) = &state.naming_screen {
        draw_naming_screen(naming, fb);
        return;
    }

    let sprite_name = match &state.phase {
        OakSpeechPhase::Greeting { .. }
        | OakSpeechPhase::Explanation { .. }
        | OakSpeechPhase::IntroduceRival { .. }
        | OakSpeechPhase::FinalSpeech { .. } => Some(("trainer", "prof.oak")),
        OakSpeechPhase::ShowNidorino { .. } => Some(("pokemon_front", "nidorino")),
        OakSpeechPhase::IntroducePlayer { .. } => Some(("player", "red")),
        OakSpeechPhase::PlayerNameChoice { .. } => Some(("player", "red")),
        OakSpeechPhase::RivalNameChoice { .. } => Some(("trainer", "rival1")),
        OakSpeechPhase::ShrinkPlayer { wait_frames } => {
            let shrink_name = if *wait_frames > 30 {
                "shrink1"
            } else {
                "shrink2"
            };
            if let Some(ref mut rm) = res {
                if let Ok(cached) = rm.load(AssetCategory::Player, shrink_name) {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
            None
        }
        OakSpeechPhase::PlayerNaming | OakSpeechPhase::RivalNaming | OakSpeechPhase::Done => None,
    };

    if let Some((category, name)) = sprite_name {
        if let Some(ref mut rm) = res {
            let result = if category == "trainer" {
                rm.load_trainer(name).ok()
            } else if category == "pokemon_front" {
                rm.load_pokemon_front(name).ok()
            } else if category == "player" {
                rm.load(AssetCategory::Player, name).ok()
            } else {
                None
            };
            if let Some(cached) = result {
                let ts = cached.tileset.clone();
                let w = cached.source_size.0;
                let h = cached.source_size.1;
                draw_centered_sprite(fb, &ts, w, h, pal);
            }
        }
    }

    match &state.phase {
        OakSpeechPhase::PlayerNameChoice { cursor } => {
            draw_text_box(fb, 4 * TILE_SIZE, 0, 11, 10, Rgba::BLACK);
            draw_text("NAME", 8 * TILE_SIZE, TILE_SIZE, Rgba::BLACK, fb);
            for (i, name) in DEFAULT_PLAYER_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { "▶" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(
                    &label,
                    6 * TILE_SIZE,
                    (3 + i as u32 * 2) * TILE_SIZE,
                    Rgba::BLACK,
                    fb,
                );
            }
        }
        OakSpeechPhase::RivalNameChoice { cursor } => {
            draw_text_box(fb, 4 * TILE_SIZE, 0, 11, 10, Rgba::BLACK);
            draw_text("NAME", 8 * TILE_SIZE, TILE_SIZE, Rgba::BLACK, fb);
            for (i, name) in DEFAULT_RIVAL_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { "▶" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(
                    &label,
                    6 * TILE_SIZE,
                    (3 + i as u32 * 2) * TILE_SIZE,
                    Rgba::BLACK,
                    fb,
                );
            }
        }
        OakSpeechPhase::Done => {
            draw_text("...", 70, 70, Rgba::BLACK, fb);
        }
        OakSpeechPhase::ShrinkPlayer { .. } => {}
        _ => {
            draw_text_box(
                fb,
                TEXT_BOX_X,
                TEXT_BOX_Y,
                TEXT_BOX_W,
                TEXT_BOX_H,
                Rgba::BLACK,
            );

            if let Some(page) = state.current_text_page() {
                let char_index = state.current_char_index();
                let (line1, line2) =
                    page.get_display_text(state.player_name.as_deref(), char_index);

                draw_text(&line1, TILE_SIZE, TEXT_BOX_Y + TILE_SIZE, Rgba::BLACK, fb);
                draw_text(
                    &line2,
                    TILE_SIZE,
                    TEXT_BOX_Y + TILE_SIZE * 3,
                    Rgba::BLACK,
                    fb,
                );
            }

            if state.is_waiting_for_input() {
                let arrow_x = 18 * TILE_SIZE;
                let arrow_y = 15 * TILE_SIZE;
                draw_text("▼", arrow_x, arrow_y, Rgba::BLACK, fb);
            }
        }
    }
}

fn draw_naming_screen(
    naming: &pokered_core::naming_screen::NamingScreenState,
    fb: &mut FrameBuffer,
) {
    draw_text("NAME ENTRY", 45, 30, Rgba::BLACK, fb);
    let name_text = format!("Name: {}_", naming.name());
    draw_text(&name_text, 30, 50, Rgba::BLACK, fb);
    let alphabet = naming.current_alphabet();
    for (row_i, row) in alphabet.iter().enumerate() {
        let y = 70 + (row_i as u32 * 12);
        let row_str: String = row.iter().map(|c| format!("{} ", c)).collect();
        draw_text(&row_str, 10, y, Rgba::BLACK, fb);
    }
    let cursor_marker = format!(
        "Cursor: row={} col={}",
        naming.cursor_row(),
        naming.cursor_col()
    );
    draw_text(&cursor_marker, 10, 134, Rgba::BLACK, fb);
}
