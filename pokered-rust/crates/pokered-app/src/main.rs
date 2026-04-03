mod audio;
mod battle_config;
mod cli;
mod direct_battle;
mod game;
mod render;
mod tools;

use clap::Parser;
use pokered_core::data::wild_data::GameVersion;
use pokered_renderer::input::InputState;
use pokered_renderer::window::{run, GameLoop, GameWindowConfig};
use pokered_renderer::{FrameBuffer, Rgba};

use crate::battle_config::BattleConfig;
use crate::cli::Cli;
use crate::direct_battle::DirectBattleGame;
use crate::game::PokemonGame;
use crate::tools::{cmd_dump_state, cmd_screenshot, cmd_screenshot_all};

fn main() {
    let cli = Cli::parse();
    let version = GameVersion::Red;

    if let Some(ref modules) = cli.debug_modules {
        if let Err(e) = pokered_core::debug_log::init("pokered-debug.log") {
            eprintln!("Warning: failed to init debug logger: {}", e);
        }
        pokered_core::debug_log::enable_from_str(modules);
    }

    match cli.command {
        None => {
            let config = GameWindowConfig {
                title: format!(
                    "Pokémon {} - Rust",
                    match version {
                        GameVersion::Red => "Red",
                        GameVersion::Blue => "Blue",
                    }
                ),
                scale: 3,
                resizable: true,
            };
            let game = PokemonGame::new(version, None, None);
            match run(config, game) {
                Ok(()) => println!("Game exited normally"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(crate::cli::Commands::Run { save, snapshot }) => {
            let config = GameWindowConfig {
                title: format!(
                    "Pokémon {} - Rust",
                    match version {
                        GameVersion::Red => "Red",
                        GameVersion::Blue => "Blue",
                    }
                ),
                scale: 3,
                resizable: true,
            };
            let game = PokemonGame::new(version, save, snapshot);
            match run(config, game) {
                Ok(()) => println!("Game exited normally"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(crate::cli::Commands::ExportSnapshot {
            ref input,
            ref output,
        }) => match PokemonGame::export_snapshot_from_sav(input.as_deref(), output) {
            Ok(()) => println!("Snapshot exported successfully"),
            Err(e) => eprintln!("Error: {}", e),
        },
        Some(crate::cli::Commands::Screenshot {
            ref screen,
            ref output,
            frames,
        }) => {
            cmd_screenshot(screen, output, frames);
        }
        Some(crate::cli::Commands::ScreenshotAll {
            ref output_dir,
            frames,
        }) => {
            cmd_screenshot_all(output_dir, frames);
        }
        Some(crate::cli::Commands::DumpState { ref screen, frames }) => {
            cmd_dump_state(screen, frames);
        }
        Some(crate::cli::Commands::Battle {
            ref config,
            ref screenshot,
            frames,
        }) => {
            let battle_config = match BattleConfig::load(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            let (player_party, enemy_party) = match battle_config.build_parties() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            let mut game = DirectBattleGame::new(
                battle_config.battle_type,
                player_party,
                enemy_party,
                battle_config.trainer_class,
            );

            if let Some(ref output_path) = screenshot {
                let input = InputState::new();
                for _ in 0..frames {
                    game.update(&input);
                }
                let mut fb = FrameBuffer::new(Rgba::WHITE);
                game.draw(&mut fb);
                fb.save_png(output_path).expect("Failed to save PNG");
                println!("Battle screenshot saved: {}", output_path.display());
            } else {
                let window_config = GameWindowConfig {
                    title: "Pokémon Battle - Direct Mode".to_string(),
                    scale: 3,
                    resizable: true,
                };
                match run(window_config, game) {
                    Ok(()) => println!("Battle finished."),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
