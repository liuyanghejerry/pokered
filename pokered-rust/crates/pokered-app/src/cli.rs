use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "pokered", about = "Pokémon Red/Blue — Rust Rewrite")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable debug logging for specific modules (comma-separated).
    /// Available modules: save, overworld, battle, menu, audio, warp, event, render, all.
    /// Example: --debug-modules save,overworld
    #[arg(long, global = true)]
    pub debug_modules: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the game in windowed mode (default)
    Run,
    /// Capture a screenshot of a specific game screen
    Screenshot {
        /// Which screen to capture
        #[arg(short, long)]
        screen: ScreenTarget,
        /// Output PNG file path
        #[arg(short, long, default_value = "screenshot.png")]
        output: PathBuf,
        /// Number of frames to advance before capturing (for animation)
        #[arg(short, long, default_value_t = 5)]
        frames: u32,
    },
    /// Capture screenshots of all game screens
    ScreenshotAll {
        /// Output directory for PNG files
        #[arg(short, long, default_value = "screenshots")]
        output_dir: PathBuf,
        /// Number of frames to advance before capturing each screen
        #[arg(short, long, default_value_t = 5)]
        frames: u32,
    },
    /// Dump game state as JSON to stdout (for comparison with PyBoy WRAM reads)
    DumpState {
        /// Which screen to transition to before dumping state
        #[arg(short, long)]
        screen: ScreenTarget,
        /// Number of frames to advance before dumping state
        #[arg(short, long, default_value_t = 0)]
        frames: u32,
    },
}

#[derive(Clone, ValueEnum)]
pub enum ScreenTarget {
    Copyright,
    Title,
    MainMenu,
    Oak,
    Overworld,
    Battle,
    StartMenu,
    Options,
    Save,
}

pub fn screen_target_to_game_screen(target: &ScreenTarget) -> pokered_core::game_state::GameScreen {
    use pokered_core::game_state::GameScreen;
    use ScreenTarget::*;

    match target {
        Copyright => GameScreen::CopyrightSplash,
        Title => GameScreen::TitleScreen,
        MainMenu => GameScreen::MainMenu,
        Oak => GameScreen::OakSpeech,
        Overworld => GameScreen::Overworld,
        Battle => GameScreen::Battle,
        StartMenu => GameScreen::StartMenu,
        Options => GameScreen::OptionsMenu,
        Save => GameScreen::SaveMenu,
    }
}

pub fn screen_name(screen: &pokered_core::game_state::GameScreen) -> &'static str {
    use pokered_core::game_state::GameScreen;

    match screen {
        GameScreen::CopyrightSplash => "copyright",
        GameScreen::TitleScreen => "title",
        GameScreen::MainMenu => "main-menu",
        GameScreen::OakSpeech => "oak",
        GameScreen::Overworld => "overworld",
        GameScreen::Battle => "battle",
        GameScreen::StartMenu => "start-menu",
        GameScreen::OptionsMenu => "options",
        GameScreen::SaveMenu => "save",
    }
}

pub const ALL_SCREENS: &[pokered_core::game_state::GameScreen] = &[
    pokered_core::game_state::GameScreen::CopyrightSplash,
    pokered_core::game_state::GameScreen::TitleScreen,
    pokered_core::game_state::GameScreen::MainMenu,
    pokered_core::game_state::GameScreen::OakSpeech,
    pokered_core::game_state::GameScreen::Overworld,
    pokered_core::game_state::GameScreen::Battle,
    pokered_core::game_state::GameScreen::StartMenu,
    pokered_core::game_state::GameScreen::OptionsMenu,
    pokered_core::game_state::GameScreen::SaveMenu,
];
