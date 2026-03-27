use pokered_core::naming_screen::{NamingScreenState, ED_CHAR, GRID_ROWS};
use pokered_core::oak_speech::{
    OakSpeechPhase, OakSpeechState, DEFAULT_PLAYER_NAMES, DEFAULT_RIVAL_NAMES,
};
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::{AssetCategory, ResourceManager};
use pokered_renderer::{FrameBuffer, Rgba, TILE_SIZE};

use super::{blit_tileset, draw_centered_sprite, draw_text_box};

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

    let is_name_choice = matches!(
        state.phase,
        OakSpeechPhase::PlayerNameChoice { .. } | OakSpeechPhase::RivalNameChoice { .. }
    );

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
                if is_name_choice {
                    let tiles_per_row = w / TILE_SIZE;
                    let sprite_x = 10 * TILE_SIZE;
                    let sprite_y = 4 * TILE_SIZE;
                    blit_tileset(fb, &ts, sprite_x, sprite_y, tiles_per_row, pal);
                } else {
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
        }
    }

    match &state.phase {
        OakSpeechPhase::PlayerNameChoice { cursor } => {
            draw_text_box(fb, 0, 0, 9, 10, Rgba::BLACK);
            draw_text("NAME", 3 * TILE_SIZE, TILE_SIZE, Rgba::BLACK, fb);
            for (i, name) in DEFAULT_PLAYER_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { "▶" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(
                    &label,
                    TILE_SIZE,
                    (2 + i as u32 * 2) * TILE_SIZE,
                    Rgba::BLACK,
                    fb,
                );
            }
            draw_text_box(
                fb,
                TEXT_BOX_X,
                TEXT_BOX_Y,
                TEXT_BOX_W,
                TEXT_BOX_H,
                Rgba::BLACK,
            );
            draw_text(
                "Your name?",
                TILE_SIZE,
                TEXT_BOX_Y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
        }
        OakSpeechPhase::RivalNameChoice { cursor } => {
            draw_text_box(fb, 0, 0, 9, 10, Rgba::BLACK);
            draw_text("NAME", 3 * TILE_SIZE, TILE_SIZE, Rgba::BLACK, fb);
            for (i, name) in DEFAULT_RIVAL_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { "▶" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(
                    &label,
                    TILE_SIZE,
                    (2 + i as u32 * 2) * TILE_SIZE,
                    Rgba::BLACK,
                    fb,
                );
            }
            draw_text_box(
                fb,
                TEXT_BOX_X,
                TEXT_BOX_Y,
                TEXT_BOX_W,
                TEXT_BOX_H,
                Rgba::BLACK,
            );
            draw_text(
                "His name?",
                TILE_SIZE,
                TEXT_BOX_Y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
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

const NAME_BOX_X: u32 = 10;
const NAME_BOX_Y: u32 = 2;
const KEYBOARD_X: u32 = 2;
const KEYBOARD_Y: u32 = 5;

pub fn draw_naming_screen(naming: &NamingScreenState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // Main text box: tile (0,4), size 18×9 (from ASM: hlcoord 0,4; b=9, c=18)
    draw_text_box(fb, 0, 4 * TILE_SIZE, 18, 9, Rgba::BLACK);

    let title = match naming.screen_type() {
        pokered_core::naming_screen::NamingScreenType::Player => "YOUR NAME?",
        pokered_core::naming_screen::NamingScreenType::Rival => "RIVAL's NAME?",
        pokered_core::naming_screen::NamingScreenType::Pokemon => "NICKNAME?",
    };
    draw_text(title, TILE_SIZE, TILE_SIZE, Rgba::BLACK, fb);

    let name = naming.name();
    let max_len = naming.max_length();

    draw_text(
        name,
        NAME_BOX_X * TILE_SIZE,
        NAME_BOX_Y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    let underscore_y = (NAME_BOX_Y + 1) * TILE_SIZE;
    let name_len = name.len() as u32;

    for i in 0..max_len as u32 {
        let ch = if i < name_len {
            "_"
        } else if i == name_len {
            "▔"
        } else {
            "_"
        };
        draw_text(
            ch,
            (NAME_BOX_X + i) * TILE_SIZE,
            underscore_y,
            Rgba::BLACK,
            fb,
        );
    }

    let alphabet = naming.current_alphabet();
    let cursor_row = naming.cursor_row();
    let cursor_col = naming.cursor_col();

    // Keyboard grid: tile (2,5), 5 rows × 9 cols (from ASM: hlcoord 2,5; lb bc,5,9)
    for (row_i, row) in alphabet.iter().enumerate() {
        let y = (KEYBOARD_Y + row_i as u32) * TILE_SIZE;
        for (col_i, &ch) in row.iter().enumerate() {
            let x = (KEYBOARD_X + col_i as u32 * 2) * TILE_SIZE;

            if row_i == cursor_row && col_i == cursor_col {
                draw_text("▶", x - TILE_SIZE, y, Rgba::BLACK, fb);
            }

            let display_ch = if ch == ED_CHAR {
                "ED"
            } else {
                match ch {
                    '×' => "x",
                    '♂' => "M",
                    '♀' => "F",
                    'é' => "e",
                    'ё' => "e",
                    _ => {
                        let mut s = String::new();
                        s.push(ch);
                        Box::leak(s.into_boxed_str())
                    }
                }
            };
            draw_text(display_ch, x, y, Rgba::BLACK, fb);
        }
    }

    // Case toggle row (row 5 = GRID_ROWS)
    let case_row_y = (KEYBOARD_Y + GRID_ROWS as u32) * TILE_SIZE;
    if cursor_row == GRID_ROWS {
        draw_text(
            "▶",
            KEYBOARD_X * TILE_SIZE - TILE_SIZE,
            case_row_y,
            Rgba::BLACK,
            fb,
        );
    }
    let case_text = if naming.is_lowercase() {
        "UPPER CASE"
    } else {
        "lower case"
    };
    draw_text(
        case_text,
        KEYBOARD_X * TILE_SIZE,
        case_row_y,
        Rgba::BLACK,
        fb,
    );

    draw_text(
        "A:Select B:Del SELECT:Case",
        TILE_SIZE,
        15 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}
