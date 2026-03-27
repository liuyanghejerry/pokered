use std::path::PathBuf;

use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::GameScreen;
use pokered_renderer::input::InputState;
use pokered_renderer::window::GameLoop;
use pokered_renderer::{FrameBuffer, Rgba};

use crate::cli::{screen_name, screen_target_to_game_screen, ScreenTarget, ALL_SCREENS};
use crate::game::PokemonGame;

pub fn capture_screen(game: &mut PokemonGame, target: GameScreen, frames: u32) -> FrameBuffer {
    game.handle_transition(target);
    let input = InputState::new();
    for _ in 0..frames {
        game.update(&input);
    }
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    game.draw(&mut fb);
    fb
}

pub fn cmd_screenshot(target: &ScreenTarget, output: &PathBuf, frames: u32) {
    let version = GameVersion::Red;
    let mut game = PokemonGame::new(version);
    let screen = screen_target_to_game_screen(target);
    println!(
        "Capturing screen: {} ({} frames)...",
        screen_name(&screen),
        frames
    );
    let fb = capture_screen(&mut game, screen, frames);
    fb.save_png(output).expect("Failed to save PNG");
    println!("Saved: {}", output.display());
}

pub fn cmd_screenshot_all(output_dir: &PathBuf, frames: u32) {
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let version = GameVersion::Red;
    let mut game = PokemonGame::new(version);
    for &screen in ALL_SCREENS {
        let name = screen_name(&screen);
        let path = output_dir.join(format!("{}.png", name));
        println!("Capturing: {}...", name);
        let fb = capture_screen(&mut game, screen, frames);
        fb.save_png(&path).expect("Failed to save PNG");
        println!("  -> {}", path.display());
    }
    println!(
        "Done. {} screenshots saved to {}",
        ALL_SCREENS.len(),
        output_dir.display()
    );
}

pub fn cmd_dump_state(target: &ScreenTarget, frames: u32) {
    let version = GameVersion::Red;
    let mut game = PokemonGame::new(version);
    let screen = screen_target_to_game_screen(target);
    game.handle_transition(screen);
    let input = InputState::new();
    for _ in 0..frames {
        game.update(&input);
    }

    let map_id = game.overworld.state.current_map as u8;
    let map_name = format!("{:?}", game.overworld.state.current_map);
    let player_x = game.overworld.state.player.x;
    let player_y = game.overworld.state.player.y;
    let screen_name_str = format!("{:?}", game.state.screen);
    let in_battle = matches!(game.state.screen, GameScreen::Battle);
    let battle_phase = format!("{:?}", game.battle.phase);
    let is_wild_battle = game.battle.is_wild;

    let state = serde_json::json!({
        "screen": screen_name_str,
        "map_id": map_id,
        "map_name": map_name,
        "player_x": player_x,
        "player_y": player_y,
        "in_battle": in_battle,
        "battle_phase": battle_phase,
        "is_wild_battle": is_wild_battle,
        "player_name": game.player_name,
        "rival_name": game.rival_name,
        "frame_count": game.frame_count,
    });
    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}
