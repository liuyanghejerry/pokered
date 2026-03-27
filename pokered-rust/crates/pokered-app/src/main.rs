mod audio;
mod cli;
mod game;
mod render;
mod tools;

use clap::Parser;
use pokered_core::data::wild_data::GameVersion;
use pokered_renderer::window::{run, GameWindowConfig};

use crate::cli::Cli;
use crate::game::PokemonGame;
use crate::tools::{cmd_dump_state, cmd_screenshot, cmd_screenshot_all};

fn main() {
    let cli = Cli::parse();
    let version = GameVersion::Red;

    match cli.command {
        None | Some(crate::cli::Commands::Run) => {
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
            let game = PokemonGame::new(version);
            match run(config, game) {
                Ok(()) => println!("Game exited normally"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
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
    }
}
