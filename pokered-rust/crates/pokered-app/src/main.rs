use pokered_core::battle::{BattleInput, BattleScreen};
use pokered_core::data::maps::MapId;
use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, ScreenAction};
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::naming_screen::NamingInput;
use pokered_core::oak_speech::{OakSpeechInput, OakSpeechResult, OakSpeechState};
use pokered_core::options_menu::{GameOptions, OptionsInput, OptionsMenuResult, OptionsMenuState};
use pokered_core::overworld::{OverworldInput, OverworldScreen};
use pokered_core::save_menu::{SaveMenuResult, SaveMenuState, SaveScreenInfo, YesNoInput};
use pokered_core::start_menu::{StartMenuAction, StartMenuInput, StartMenuState};
use pokered_core::title_screen::TitleScreenState;
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::window::{run, GameLoop, GameWindowConfig};
use pokered_renderer::{FrameBuffer, Rgba};

struct PokemonGame {
    state: GameState,
    title_screen: TitleScreenState,
    main_menu: MainMenuState,
    oak_speech: OakSpeechState,
    overworld: OverworldScreen,
    battle: BattleScreen,
    start_menu: StartMenuState,
    options_menu: OptionsMenuState,
    save_menu: SaveMenuState,
    player_name: String,
    rival_name: String,
    frame_count: u64,
    exit_requested: bool,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(None);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown);
        let battle = BattleScreen::new(true);
        let start_menu = StartMenuState::new(false, false);
        let options_menu = OptionsMenuState::new(GameOptions::default());
        let save_menu = SaveMenuState::new(
            SaveScreenInfo {
                player_name: "RED".to_string(),
                num_badges: 0,
                pokedex_owned: 0,
                play_time_hours: 0,
                play_time_minutes: 0,
            },
            false,
            false,
        );
        Self {
            state,
            title_screen,
            main_menu,
            oak_speech,
            overworld,
            battle,
            start_menu,
            options_menu,
            save_menu,
            player_name: "RED".to_string(),
            rival_name: "BLUE".to_string(),
            frame_count: 0,
            exit_requested: false,
        }
    }

    fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => {
                self.title_screen.reset();
            }
            GameScreen::MainMenu => {
                self.main_menu = MainMenuState::new(None);
            }
            GameScreen::OakSpeech => {
                self.oak_speech = OakSpeechState::new();
            }
            GameScreen::Overworld => {
                self.overworld = OverworldScreen::new(MapId::PalletTown);
            }
            GameScreen::Battle => {
                self.battle = BattleScreen::new(true);
            }
            GameScreen::StartMenu => {
                self.start_menu.open(false, false);
            }
            GameScreen::OptionsMenu => {
                self.options_menu = OptionsMenuState::new(GameOptions::default());
            }
            GameScreen::SaveMenu => {
                self.save_menu = SaveMenuState::new(
                    SaveScreenInfo {
                        player_name: self.player_name.clone(),
                        num_badges: 0,
                        pokedex_owned: 0,
                        play_time_hours: 0,
                        play_time_minutes: 0,
                    },
                    false,
                    false,
                );
            }
            GameScreen::CopyrightSplash => {
                self.title_screen.reset();
            }
        }
        self.state.transition_to(screen);
    }
}

impl GameLoop for PokemonGame {
    fn update(&mut self, input: &InputState) {
        self.frame_count += 1;

        let action = match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                let any_pressed = input.any_just_pressed();
                self.title_screen.update_frame(any_pressed)
            }
            GameScreen::MainMenu => {
                let menu_input = MenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A) || input.is_just_pressed(GbButton::Start),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.main_menu.update_frame(menu_input)
            }
            GameScreen::OakSpeech => {
                if self.oak_speech.is_naming_active() {
                    let naming_input = NamingInput {
                        up: input.is_just_pressed(GbButton::Up),
                        down: input.is_just_pressed(GbButton::Down),
                        left: input.is_just_pressed(GbButton::Left),
                        right: input.is_just_pressed(GbButton::Right),
                        a: input.is_just_pressed(GbButton::A),
                        b: input.is_just_pressed(GbButton::B),
                        start: input.is_just_pressed(GbButton::Start),
                        select: input.is_just_pressed(GbButton::Select),
                    };
                    match self.oak_speech.update_naming_frame(naming_input) {
                        OakSpeechResult::PlayerNameSet(name) => {
                            self.player_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::RivalNameSet(name) => {
                            self.rival_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::Finished => {
                            ScreenAction::Transition(GameScreen::Overworld)
                        }
                        OakSpeechResult::Active => ScreenAction::Continue,
                    }
                } else {
                    let oak_input = OakSpeechInput {
                        up: input.is_just_pressed(GbButton::Up),
                        down: input.is_just_pressed(GbButton::Down),
                        a: input.is_just_pressed(GbButton::A),
                        b: input.is_just_pressed(GbButton::B),
                    };
                    match self.oak_speech.update_frame(oak_input) {
                        OakSpeechResult::PlayerNameSet(name) => {
                            self.player_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::RivalNameSet(name) => {
                            self.rival_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::Finished => {
                            ScreenAction::Transition(GameScreen::Overworld)
                        }
                        OakSpeechResult::Active => ScreenAction::Continue,
                    }
                }
            }
            GameScreen::Overworld => {
                let ow_input = OverworldInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                    select: input.is_just_pressed(GbButton::Select),
                };
                self.overworld.update_frame(ow_input)
            }
            GameScreen::Battle => {
                let battle_input = BattleInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.battle.update_frame(battle_input)
            }
            GameScreen::StartMenu => {
                let sm_input = StartMenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                };
                match self.start_menu.update_frame(sm_input) {
                    StartMenuAction::Close => ScreenAction::Transition(GameScreen::Overworld),
                    StartMenuAction::OpenOption => {
                        ScreenAction::Transition(GameScreen::OptionsMenu)
                    }
                    StartMenuAction::OpenSave => ScreenAction::Transition(GameScreen::SaveMenu),
                    _ => ScreenAction::Continue,
                }
            }
            GameScreen::OptionsMenu => {
                let opt_input = OptionsInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                };
                match self.options_menu.tick(opt_input) {
                    OptionsMenuResult::Closed => ScreenAction::Transition(GameScreen::StartMenu),
                    OptionsMenuResult::Active => ScreenAction::Continue,
                }
            }
            GameScreen::SaveMenu => {
                let save_input = YesNoInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                };
                match self.save_menu.tick(save_input) {
                    SaveMenuResult::Saved | SaveMenuResult::Cancelled => {
                        ScreenAction::Transition(GameScreen::StartMenu)
                    }
                    SaveMenuResult::Active => ScreenAction::Continue,
                }
            }
        };

        if let ScreenAction::Transition(new_screen) = action {
            self.handle_transition(new_screen);
        }
    }

    fn draw(&mut self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.clear(Rgba::WHITE);

        match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, frame_buffer);
            }
            GameScreen::MainMenu => {
                draw_main_menu(&self.main_menu, frame_buffer);
            }
            GameScreen::OakSpeech => {
                draw_oak_speech(&self.oak_speech, frame_buffer);
            }
            GameScreen::Overworld => {
                draw_overworld(&self.overworld, frame_buffer);
            }
            GameScreen::Battle => {
                draw_battle(&self.battle, frame_buffer);
            }
            GameScreen::StartMenu => {
                draw_start_menu(&self.start_menu, &self.player_name, frame_buffer);
            }
            GameScreen::OptionsMenu => {
                draw_options_menu(&self.options_menu, frame_buffer);
            }
            GameScreen::SaveMenu => {
                draw_save_menu(&self.save_menu, frame_buffer);
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }
}

fn draw_title_screen(state: &TitleScreenState, fb: &mut FrameBuffer) {
    let fade = state.fade_progress();
    let bg_color = if fade > 0.0 {
        let v = (255.0 * (1.0 - fade)) as u8;
        Rgba::rgb(v, v, v)
    } else {
        Rgba::WHITE
    };
    fb.clear(bg_color);

    let phase_text = format!("Title Screen: {:?}", state.phase);
    draw_text(&phase_text, 10, 10, Rgba::BLACK, fb);

    let mon_text = format!("Current Mon: {:?}", state.current_mon);
    draw_text(&mon_text, 10, 30, Rgba::BLACK, fb);

    let scroll_text = format!("Scroll Y: {}", state.scroll_y);
    draw_text(&scroll_text, 10, 50, Rgba::BLACK, fb);

    draw_text("Press any button to continue", 10, 100, Rgba::BLACK, fb);
    draw_text("Press ESC to exit", 10, 115, Rgba::BLACK, fb);
}

fn draw_main_menu(state: &MainMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("MAIN MENU", 60, 20, Rgba::BLACK, fb);
    let labels = state.item_labels();
    for (i, label) in labels.iter().enumerate() {
        let y = 50 + (i as u32 * 20);
        let prefix = if i == state.cursor { "> " } else { "  " };
        let text = format!("{}{}", prefix, label);
        draw_text(&text, 50, y, Rgba::BLACK, fb);
    }
    draw_text("Up/Down: Select", 10, 120, Rgba::BLACK, fb);
    draw_text("A/Start: Confirm", 10, 130, Rgba::BLACK, fb);
}

fn draw_oak_speech(state: &OakSpeechState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("OAK'S SPEECH", 40, 10, Rgba::BLACK, fb);

    if let Some(naming) = &state.naming_screen {
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
    } else if let Some(text) = state.current_intro_text() {
        draw_text(text, 20, 60, Rgba::BLACK, fb);
    } else {
        let phase_text = format!("{:?}", state.phase);
        draw_text(&phase_text, 10, 60, Rgba::BLACK, fb);
    }
}

fn draw_overworld(screen: &OverworldScreen, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    let pos_text = format!(
        "Map: {:?}  Pos: ({}, {})",
        screen.state.current_map, screen.state.player.x, screen.state.player.y
    );
    draw_text(&pos_text, 5, 10, Rgba::BLACK, fb);

    let facing_text = format!("Facing: {:?}", screen.state.player.facing);
    draw_text(&facing_text, 5, 30, Rgba::BLACK, fb);

    draw_text("OVERWORLD", 50, 60, Rgba::BLACK, fb);
    draw_text("Arrows: Move", 10, 100, Rgba::BLACK, fb);
    draw_text("Start: Menu", 10, 115, Rgba::BLACK, fb);
}

fn draw_battle(screen: &BattleScreen, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("BATTLE", 55, 10, Rgba::BLACK, fb);

    let phase_text = format!("Phase: {:?}", screen.phase);
    draw_text(&phase_text, 10, 30, Rgba::BLACK, fb);

    if matches!(screen.phase, pokered_core::battle::BattlePhase::PlayerMenu) {
        let menu_labels = ["FIGHT", "BAG", "POKeMON", "RUN"];
        for (i, label) in menu_labels.iter().enumerate() {
            let row = i / 2;
            let col = i % 2;
            let x = 20 + (col as u32 * 70);
            let y = 100 + (row as u32 * 20);
            let is_selected = screen.battle_menu.row() == row && screen.battle_menu.col() == col;
            let prefix = if is_selected { ">" } else { " " };
            let text = format!("{}{}", prefix, label);
            draw_text(&text, x, y, Rgba::BLACK, fb);
        }
    }
}

fn draw_start_menu(state: &StartMenuState, player_name: &str, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("START MENU", 50, 10, Rgba::BLACK, fb);

    let labels = state.item_labels(player_name);
    for (i, label) in labels.iter().enumerate() {
        let y = 30 + (i as u32 * 16);
        let prefix = if i == state.cursor() { "> " } else { "  " };
        let text = format!("{}{}", prefix, label.as_str());
        draw_text(&text, 20, y, Rgba::BLACK, fb);
    }
}

fn draw_options_menu(state: &OptionsMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("OPTIONS", 55, 10, Rgba::BLACK, fb);

    let speed_text = format!("TEXT SPEED: {:?}", state.options.text_speed);
    draw_text(&speed_text, 10, 40, Rgba::BLACK, fb);

    let anim_text = format!("BATTLE ANIM: {:?}", state.options.battle_animation);
    draw_text(&anim_text, 10, 60, Rgba::BLACK, fb);

    let style_text = format!("BATTLE STYLE: {:?}", state.options.battle_style);
    draw_text(&style_text, 10, 80, Rgba::BLACK, fb);

    let row_text = format!("Cursor: {:?}", state.row);
    draw_text(&row_text, 10, 110, Rgba::BLACK, fb);

    draw_text("B: Close", 10, 130, Rgba::BLACK, fb);
}

fn draw_save_menu(state: &SaveMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("SAVE GAME", 50, 10, Rgba::BLACK, fb);

    let phase_text = format!("Phase: {:?}", state.phase);
    draw_text(&phase_text, 10, 40, Rgba::BLACK, fb);

    let cursor_text = format!("Cursor: {:?}", state.cursor);
    draw_text(&cursor_text, 10, 60, Rgba::BLACK, fb);

    let info = format!(
        "{} Badges:{} Dex:{}",
        state.info.player_name, state.info.num_badges, state.info.pokedex_owned
    );
    draw_text(&info, 10, 80, Rgba::BLACK, fb);

    let time = format!(
        "Time: {:02}:{:02}",
        state.info.play_time_hours, state.info.play_time_minutes
    );
    draw_text(&time, 10, 100, Rgba::BLACK, fb);
}

fn main() {
    let version = GameVersion::Red;

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
