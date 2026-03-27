use pokered_core::main_menu::MainMenuState;
use pokered_core::options_menu::OptionsMenuState;
use pokered_core::save_menu::SaveMenuState;
use pokered_core::start_menu::StartMenuState;
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::{FrameBuffer, Rgba, TILE_SIZE};

use super::draw_text_box;

pub fn draw_main_menu(state: &MainMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    let labels = state.item_labels();
    let num_items = labels.len();
    let box_h = if num_items > 2 { 6_u32 } else { 4_u32 };

    draw_text_box(fb, 0, 0, 13, box_h, Rgba::BLACK);

    for (i, label) in labels.iter().enumerate() {
        let tile_y = 2 + (i as u32 * 2);
        let px_x = 2 * TILE_SIZE;
        let px_y = tile_y * TILE_SIZE;
        draw_text(label, px_x, px_y, Rgba::BLACK, fb);
    }

    let cursor_tile_y = 2 + (state.cursor as u32 * 2);
    draw_text(
        "\x7F",
        1 * TILE_SIZE,
        cursor_tile_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

pub fn draw_start_menu(state: &StartMenuState, player_name: &str, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    let labels = state.item_labels(player_name);
    let box_h = (labels.len() as u32) * 2;

    draw_text_box(fb, 10 * TILE_SIZE, 0, 8, box_h, Rgba::BLACK);

    for (i, label) in labels.iter().enumerate() {
        let tile_y = 2 + (i as u32 * 2);
        draw_text(
            label.as_str(),
            12 * TILE_SIZE,
            tile_y * TILE_SIZE,
            Rgba::BLACK,
            fb,
        );
    }

    let cursor_tile_y = 2 + (state.cursor() as u32 * 2);
    draw_text(
        ">",
        11 * TILE_SIZE,
        cursor_tile_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

pub fn draw_options_menu(state: &OptionsMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    draw_text_box(fb, 0, 0 * TILE_SIZE, 18, 3, Rgba::BLACK);
    draw_text_box(fb, 0, 5 * TILE_SIZE, 18, 3, Rgba::BLACK);
    draw_text_box(fb, 0, 10 * TILE_SIZE, 18, 3, Rgba::BLACK);

    draw_text("TEXT SPEED", 1 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    let speed_str = match state.options.text_speed {
        pokered_core::options_menu::TextSpeed::Fast => " FAST  MEDIUM SLOW",
        pokered_core::options_menu::TextSpeed::Medium => " FAST  MEDIUM SLOW",
        pokered_core::options_menu::TextSpeed::Slow => " FAST  MEDIUM SLOW",
    };
    draw_text(speed_str, 1 * TILE_SIZE, 3 * TILE_SIZE, Rgba::BLACK, fb);

    let speed_cursor_x = match state.options.text_speed {
        pokered_core::options_menu::TextSpeed::Fast => 1,
        pokered_core::options_menu::TextSpeed::Medium => 7,
        pokered_core::options_menu::TextSpeed::Slow => 14,
    };
    draw_text(
        ">",
        speed_cursor_x * TILE_SIZE,
        3 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    draw_text(
        "BATTLE ANIMATION",
        1 * TILE_SIZE,
        6 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        " ON       OFF",
        1 * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let anim_cursor_x = match state.options.battle_animation {
        pokered_core::options_menu::BattleAnimation::On => 1,
        pokered_core::options_menu::BattleAnimation::Off => 10,
    };
    draw_text(
        ">",
        anim_cursor_x * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    draw_text(
        "BATTLE STYLE",
        1 * TILE_SIZE,
        11 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        " SHIFT    SET",
        1 * TILE_SIZE,
        13 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let style_cursor_x = match state.options.battle_style {
        pokered_core::options_menu::BattleStyle::Shift => 1,
        pokered_core::options_menu::BattleStyle::Set => 10,
    };
    draw_text(
        ">",
        style_cursor_x * TILE_SIZE,
        13 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    draw_text("CANCEL", 2 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);
}

pub fn draw_save_menu(state: &SaveMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    draw_text_box(fb, 4 * TILE_SIZE, 0, 14, 8, Rgba::BLACK);

    draw_text("PLAYER", 5 * TILE_SIZE, 2 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text(
        &state.info.player_name,
        12 * TILE_SIZE,
        2 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    draw_text("BADGES", 5 * TILE_SIZE, 4 * TILE_SIZE, Rgba::BLACK, fb);
    let badges = format!("{}", state.info.num_badges);
    draw_text(&badges, 17 * TILE_SIZE, 4 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text("#DEX", 5 * TILE_SIZE, 6 * TILE_SIZE, Rgba::BLACK, fb);
    let dex = format!("{}", state.info.pokedex_owned);
    draw_text(&dex, 16 * TILE_SIZE, 6 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text("TIME", 5 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    let time = format!(
        "{:>3}:{:02}",
        state.info.play_time_hours, state.info.play_time_minutes
    );
    draw_text(&time, 13 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text_box(fb, 0, 11 * TILE_SIZE, 18, 4, Rgba::BLACK);
    draw_text(
        "Would you like to",
        1 * TILE_SIZE,
        12 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        "SAVE the game?",
        1 * TILE_SIZE,
        14 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    let yes_no_x = 15_u32;
    draw_text_box(fb, yes_no_x * TILE_SIZE, 7 * TILE_SIZE, 3, 2, Rgba::BLACK);
    draw_text(
        "YES",
        (yes_no_x + 1) * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        "NO",
        (yes_no_x + 1) * TILE_SIZE,
        9 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let cursor_y = if state.cursor == pokered_core::save_menu::YesNoChoice::Yes {
        8
    } else {
        9
    };
    draw_text(
        ">",
        yes_no_x * TILE_SIZE,
        cursor_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}
