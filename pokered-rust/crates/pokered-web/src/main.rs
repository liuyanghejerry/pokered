//! Pokémon Red/Blue - Web/WASM Build
//!
//! This crate provides a WebAssembly-compatible build of the game,
//! using pixels + winit with async initialization for wasm32 targets.
//! Also runs natively as a fallback for development.

use std::sync::Arc;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

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
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: u32 = 3;
const BLACK_SCREEN_DURATION: u32 = 30;

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
    black_screen_frames: u32,
    pending_screen: Option<GameScreen>,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(None);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown);
        let battle = BattleScreen::new(true);
        let start_menu = StartMenuState::new(false, false, false);
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
            black_screen_frames: 0,
            pending_screen: None,
        }
    }

    fn update(&mut self, input: &InputState) {
        self.frame_count += 1;

        if self.black_screen_frames > 0 {
            self.black_screen_frames -= 1;
            if self.black_screen_frames == 0 {
                if let Some(screen) = self.pending_screen.take() {
                    self.handle_transition(screen);
                }
            }
            return;
        }

        let action = match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                let any_pressed = input.any_just_pressed();
                self.title_screen.update_frame(any_pressed)
            }
            GameScreen::MainMenu => {
                let menu_input = MenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A)
                        || input.is_just_pressed(GbButton::Start),
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
                    StartMenuAction::Close => {
                        ScreenAction::Transition(GameScreen::Overworld)
                    }
                    StartMenuAction::OpenOption => {
                        ScreenAction::Transition(GameScreen::OptionsMenu)
                    }
                    StartMenuAction::OpenSave => {
                        ScreenAction::Transition(GameScreen::SaveMenu)
                    }
                    StartMenuAction::OpenItem
                    | StartMenuAction::OpenPokedex
                    | StartMenuAction::OpenPokemon
                    | StartMenuAction::OpenTrainerInfo => {
                        self.start_menu.redisplay();
                        ScreenAction::Continue
                    }
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
                    OptionsMenuResult::Closed => {
                        ScreenAction::Transition(GameScreen::StartMenu)
                    }
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
            use pokered_core::game_state::MainMenuChoice;
            let needs_black_screen = new_screen == GameScreen::Overworld
                && self.state.screen == GameScreen::MainMenu
                && self.main_menu.last_choice == Some(MainMenuChoice::Continue);

            if needs_black_screen {
                self.black_screen_frames = BLACK_SCREEN_DURATION;
                self.pending_screen = Some(new_screen);
            } else {
                self.handle_transition(new_screen);
            }
        }
    }

    fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => self.title_screen.reset(),
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
                self.start_menu.open(false, false, false);
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
            GameScreen::CopyrightSplash => self.title_screen.reset(),
        }
        self.state.transition_to(screen);
    }

    fn draw(&self, fb: &mut FrameBuffer) {
        if self.black_screen_frames > 0 {
            fb.clear(Rgba::BLACK);
            return;
        }

        fb.clear(Rgba::WHITE);
        match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, fb);
            }
            GameScreen::MainMenu => draw_main_menu(&self.main_menu, fb),
            GameScreen::OakSpeech => draw_oak_speech(&self.oak_speech, fb),
            GameScreen::Overworld => draw_overworld(&self.overworld, fb),
            GameScreen::Battle => draw_battle(&self.battle, fb),
            GameScreen::StartMenu => {
                draw_overworld(&self.overworld, fb);
                draw_start_menu(&self.start_menu, &self.player_name, fb)
            }
            GameScreen::OptionsMenu => draw_options_menu(&self.options_menu, fb),
            GameScreen::SaveMenu => {
                draw_overworld(&self.overworld, fb);
                draw_save_menu(&self.save_menu, fb);
            }
        }
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
    draw_text("[Web Build]", 10, 130, Rgba::BLACK, fb);
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
    } else if let Some(text) = state.current_intro_text() {
        draw_text(&text, 20, 60, Rgba::BLACK, fb);
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
    draw_text("[Web Build]", 10, 130, Rgba::BLACK, fb);
}

fn draw_battle(screen: &BattleScreen, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("BATTLE", 55, 10, Rgba::BLACK, fb);
    let phase_text = format!("Phase: {:?}", screen.phase);
    draw_text(&phase_text, 10, 30, Rgba::BLACK, fb);
    if matches!(screen.phase, pokered_core::battle::BattlePhase::PlayerMenu) {
        let labels = ["FIGHT", "BAG", "POKeMON", "RUN"];
        for (i, label) in labels.iter().enumerate() {
            let row = i / 2;
            let col = i % 2;
            let x = 20 + (col as u32 * 70);
            let y = 100 + (row as u32 * 20);
            let sel = screen.battle_menu.row() == row && screen.battle_menu.col() == col;
            let prefix = if sel { ">" } else { " " };
            draw_text(&format!("{}{}", prefix, label), x, y, Rgba::BLACK, fb);
        }
    }
}

fn draw_start_menu(state: &StartMenuState, player_name: &str, fb: &mut FrameBuffer) {
    draw_text("START MENU", 50, 10, Rgba::BLACK, fb);
    let labels = state.item_labels(player_name);
    for (i, label) in labels.iter().enumerate() {
        let y = 30 + (i as u32 * 16);
        let prefix = if i == state.cursor() { "> " } else { "  " };
        draw_text(&format!("{}{}", prefix, label.as_str()), 20, y, Rgba::BLACK, fb);
    }
}

fn draw_options_menu(state: &OptionsMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text("OPTIONS", 55, 10, Rgba::BLACK, fb);
    draw_text(&format!("TEXT SPEED: {:?}", state.options.text_speed), 10, 40, Rgba::BLACK, fb);
    draw_text(&format!("BATTLE ANIM: {:?}", state.options.battle_animation), 10, 60, Rgba::BLACK, fb);
    draw_text(&format!("BATTLE STYLE: {:?}", state.options.battle_style), 10, 80, Rgba::BLACK, fb);
    draw_text(&format!("Cursor: {:?}", state.row), 10, 110, Rgba::BLACK, fb);
}

fn draw_save_menu(state: &SaveMenuState, fb: &mut FrameBuffer) {
    draw_text("SAVE GAME", 50, 10, Rgba::BLACK, fb);
    draw_text(&format!("Phase: {:?}", state.phase), 10, 40, Rgba::BLACK, fb);
    draw_text(&format!("Cursor: {:?}", state.cursor), 10, 60, Rgba::BLACK, fb);
    draw_text(
        &format!("{} Badges:{}", state.info.player_name, state.info.num_badges),
        10, 80, Rgba::BLACK, fb,
    );
}

#[cfg(target_arch = "wasm32")]
fn get_window_size() -> LogicalSize<f64> {
    let client_window = web_sys::window().unwrap();
    let vw = client_window.inner_width().unwrap().as_f64().unwrap();
    let vh = client_window.inner_height().unwrap().as_f64().unwrap();
    let game_w = (SCREEN_WIDTH * SCALE) as f64;
    let game_h = (SCREEN_HEIGHT * SCALE) as f64;
    let w = vw.min(game_w);
    let h = vh.min(game_h);
    LogicalSize::new(w, h)
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("error initializing logger");
        wasm_bindgen_futures::spawn_local(run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
}

async fn run() {
    let version = GameVersion::Red;
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(
            (SCREEN_WIDTH * SCALE) as f64,
            (SCREEN_HEIGHT * SCALE) as f64,
        );
        #[allow(deprecated)]
        Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&format!(
                            "Pokémon {} - Rust (Web)",
                            match version {
                                GameVersion::Red => "Red",
                                GameVersion::Blue => "Blue",
                            }
                        ))
                        .with_inner_size(size)
                        .with_min_inner_size(LogicalSize::new(
                            SCREEN_WIDTH as f64,
                            SCREEN_HEIGHT as f64,
                        ))
                        .with_resizable(true),
                )
                .unwrap(),
        )
    };

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas().unwrap()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        let resize_window = Arc::clone(&window);
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
            move |_e: web_sys::Event| {
                let _ = resize_window.request_inner_size(get_window_size());
            },
        ) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        let _ = window.request_inner_size(get_window_size());
    }

    let mut pixels = {
        #[cfg(not(target_arch = "wasm32"))]
        let window_size = window.inner_size();

        #[cfg(target_arch = "wasm32")]
        let window_size = get_window_size().to_physical::<u32>(window.scale_factor());

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        let builder = PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture);

        #[cfg(target_arch = "wasm32")]
        let builder = {
            let texture_format = pixels::wgpu::TextureFormat::Rgba8Unorm;
            builder
                .texture_format(texture_format)
                .surface_texture_format(texture_format)
        };

        builder.build_async().await.expect("Pixels error")
    };

    let mut game = PokemonGame::new(version);
    let mut frame_buffer = FrameBuffer::default();
    let mut input = InputState::new();

    #[allow(deprecated)]
    let res = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => {
                game.draw(&mut frame_buffer);
                pixels.frame_mut().copy_from_slice(&frame_buffer.data);
                if let Err(err) = pixels.render() {
                    log_error("pixels.render", err);
                    elwt.exit();
                }
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        elwt.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                if let PhysicalKey::Code(keycode) = key_event.physical_key {
                    let pressed = key_event.state == ElementState::Pressed;
                    if pressed && keycode == KeyCode::Escape {
                        elwt.exit();
                        return;
                    }
                    input.set_from_keycode(keycode, pressed);
                }
            }
            _ => {}
        },
        Event::AboutToWait => {
            game.update(&input);
            input.begin_frame();
            window.request_redraw();
        }
        _ => {}
    });

    res.unwrap();
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
