//! Pokémon Red/Blue - Rust Rewrite
//!
//! Main entry point that connects the game state to the renderer.

use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, ScreenAction};
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::title_screen::TitleScreenState;
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::window::{run, GameLoop, GameWindowConfig};
use pokered_renderer::{FrameBuffer, Rgba};

/// Main game struct that owns all game state and implements GameLoop.
struct PokemonGame {
    /// Top-level game state (screen, config, save).
    state: GameState,
    /// Title screen state (animation phase, current mon, etc.).
    title_screen: TitleScreenState,
    /// Main menu state (cursor position, options, etc.).
    main_menu: MainMenuState,
    /// Frame counter for debugging/general timing.
    frame_count: u64,
    /// Flag to request exit.
    exit_requested: bool,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(None); // No save file for now
        Self {
            state,
            title_screen,
            main_menu,
            frame_count: 0,
            exit_requested: false,
        }
    }

    /// Handle screen transitions returned from sub-screens.
    fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => {
                self.title_screen.reset();
            }
            GameScreen::MainMenu => {
                self.main_menu = MainMenuState::new(None);
            }
            GameScreen::Overworld => {
                // TODO: Initialize overworld state
            }
            GameScreen::Battle => {
                // TODO: Initialize battle state
            }
            GameScreen::CopyrightSplash => {
                self.title_screen.reset();
            }
            GameScreen::OakSpeech => {
                // TODO: Initialize Oak speech state
            }
        }
        self.state.transition_to(screen);
    }
}

impl GameLoop for PokemonGame {
    fn update(&mut self, input: &InputState) {
        self.frame_count += 1;

        // Update based on current screen
        let action = match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                // Any button press on title screen
                let any_pressed = input.any_just_pressed();
                self.title_screen.update_frame(any_pressed)
            }
            GameScreen::MainMenu => {
                // Handle main menu input using MenuInput struct
                let menu_input = MenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A) || input.is_just_pressed(GbButton::Start),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.main_menu.update_frame(menu_input)
            }
            GameScreen::OakSpeech => {
                // TODO: Implement Oak speech logic
                ScreenAction::Continue
            }
            GameScreen::Overworld => {
                // TODO: Implement overworld logic
                ScreenAction::Continue
            }
            GameScreen::Battle => {
                // TODO: Implement battle logic
                ScreenAction::Continue
            }
        };

        // Handle screen transitions
        if let ScreenAction::Transition(new_screen) = action {
            self.handle_transition(new_screen);
        }
    }

    fn draw(&mut self, frame_buffer: &mut FrameBuffer) {
        // Clear to white (Game Boy default background)
        frame_buffer.clear(Rgba::WHITE);

        // Draw based on current screen
        match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, frame_buffer);
            }
            GameScreen::MainMenu => {
                draw_main_menu(&self.main_menu, frame_buffer);
            }
            GameScreen::OakSpeech => {
                // TODO: Draw Oak speech
                draw_placeholder("Oak Speech", frame_buffer);
            }
            GameScreen::Overworld => {
                // TODO: Draw overworld
                draw_placeholder("Overworld", frame_buffer);
            }
            GameScreen::Battle => {
                // TODO: Draw battle
                draw_placeholder("Battle", frame_buffer);
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }
}

/// Draw the title screen.
fn draw_title_screen(state: &TitleScreenState, fb: &mut FrameBuffer) {
    // Background color based on fade
    let fade = state.fade_progress();
    let bg_color = if fade > 0.0 {
        let v = (255.0 * (1.0 - fade)) as u8;
        Rgba::rgb(v, v, v)
    } else {
        Rgba::WHITE
    };
    fb.clear(bg_color);

    // Draw placeholder for now - just show which screen we're on
    let phase_text = format!("Title Screen: {:?}", state.phase);
    draw_text(&phase_text, 10, 10, Rgba::BLACK, fb);

    let mon_text = format!("Current Mon: {:?}", state.current_mon);
    draw_text(&mon_text, 10, 30, Rgba::BLACK, fb);

    let scroll_text = format!("Scroll Y: {}", state.scroll_y);
    draw_text(&scroll_text, 10, 50, Rgba::BLACK, fb);

    // Instructions
    draw_text("Press any button to continue", 10, 100, Rgba::BLACK, fb);
    draw_text("Press ESC to exit", 10, 115, Rgba::BLACK, fb);
}

/// Draw the main menu.
fn draw_main_menu(state: &MainMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // Draw menu title
    draw_text("MAIN MENU", 60, 20, Rgba::BLACK, fb);

    // Draw menu options using item_labels()
    let labels = state.item_labels();
    for (i, label) in labels.iter().enumerate() {
        let y = 50 + (i as u32 * 20);
        let prefix = if i == state.cursor { "> " } else { "  " };
        let text = format!("{}{}", prefix, label);
        draw_text(&text, 50, y, Rgba::BLACK, fb);
    }

    // Instructions
    draw_text("Up/Down: Select", 10, 120, Rgba::BLACK, fb);
    draw_text("A/Start: Confirm", 10, 130, Rgba::BLACK, fb);
}

/// Draw a placeholder screen with text.
fn draw_placeholder(name: &str, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text(name, 10, 60, Rgba::BLACK, fb);
    draw_text("Not yet implemented", 10, 80, Rgba::BLACK, fb);
}

fn main() {
    // Game version - could be configurable via command line
    let version = GameVersion::Red;

    // Create window configuration
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

    // Create game instance
    let game = PokemonGame::new(version);

    // Run the game loop
    match run(config, game) {
        Ok(()) => println!("Game exited normally"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
