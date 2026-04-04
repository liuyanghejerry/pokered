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

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use pokered_core::battle::{BattleInput, BattleScreen};
use pokered_core::data::maps::MapId;
use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, ScreenAction};
use pokered_core::intro_scene::IntroSceneState;
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::naming_screen::NamingInput;
use pokered_core::oak_speech::{OakSpeechInput, OakSpeechResult, OakSpeechState};
use pokered_core::options_menu::{GameOptions, OptionsInput, OptionsMenuResult, OptionsMenuState};
use pokered_core::overworld::{OverworldInput, OverworldScreen};
use pokered_core::save_menu::{SaveMenuResult, SaveMenuState, SaveScreenInfo, YesNoInput};
use pokered_core::start_menu::{StartMenuAction, StartMenuInput, StartMenuState};
use pokered_core::title_screen::TitleScreenState;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::resource::ResourceManager;
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH};

#[path = "../../pokered-app/src/render/mod.rs"]
mod app_render;
use app_render::{
    draw_battle, draw_intro_scene, draw_main_menu, draw_oak_speech, draw_options_menu,
    draw_overworld, draw_save_menu, draw_start_menu, draw_title_screen, BattleVisualEffects,
};

const SCALE: u32 = 3;
const BLACK_SCREEN_DURATION: u32 = 30;

struct PokemonGame {
    state: GameState,
    title_screen: TitleScreenState,
    intro_scene: IntroSceneState,
    main_menu: MainMenuState,
    oak_speech: OakSpeechState,
    overworld: OverworldScreen,
    battle: BattleScreen,
    start_menu: StartMenuState,
    options_menu: OptionsMenuState,
    save_menu: SaveMenuState,
    battle_vfx: BattleVisualEffects,
    player_name: String,
    rival_name: String,
    frame_count: u64,
    black_screen_frames: u32,
    pending_screen: Option<GameScreen>,
    resources: Option<ResourceManager>,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let intro_scene = IntroSceneState::new();
        let main_menu = MainMenuState::new(None);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown, None);
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

        #[cfg(target_arch = "wasm32")]
        let resources = None;

        #[cfg(not(target_arch = "wasm32"))]
        let resources = pokered_renderer::resource::AssetRoot::auto_detect()
            .ok()
            .map(ResourceManager::new);

        Self {
            state,
            title_screen,
            intro_scene,
            main_menu,
            oak_speech,
            overworld,
            battle,
            start_menu,
            options_menu,
            save_menu,
            battle_vfx: BattleVisualEffects::default(),
            player_name: "RED".to_string(),
            rival_name: "BLUE".to_string(),
            frame_count: 0,
            black_screen_frames: 0,
            pending_screen: None,
            resources,
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
            GameScreen::IntroScene => {
                let any_pressed = input.any_just_pressed();
                self.intro_scene.update_frame(any_pressed)
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
                self.overworld = OverworldScreen::new(MapId::PalletTown, None);
            }
            GameScreen::Battle => {
                self.battle = BattleScreen::new(true);
                self.battle_vfx = BattleVisualEffects::default();
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
            GameScreen::IntroScene => self.intro_scene.reset(),
        }
        self.state.transition_to(screen);
    }

    fn draw(&mut self, fb: &mut FrameBuffer) {
        if self.black_screen_frames > 0 {
            fb.clear(Rgba::BLACK);
            return;
        }

        fb.clear(Rgba::WHITE);
        match self.state.screen {
            GameScreen::CopyrightSplash => {
                draw_title_screen(&self.title_screen, true, &mut self.resources, fb);
            }
            GameScreen::IntroScene => {
                draw_intro_scene(&self.intro_scene, &mut self.resources, fb);
            }
            GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, false, &mut self.resources, fb);
            }
            GameScreen::MainMenu => draw_main_menu(&self.main_menu, fb),
            GameScreen::OakSpeech => draw_oak_speech(&self.oak_speech, &mut self.resources, fb),
            GameScreen::Overworld => draw_overworld(&self.overworld, &mut self.resources, fb),
            GameScreen::Battle => draw_battle(&self.battle, &mut self.resources, fb, &mut self.battle_vfx),
            GameScreen::StartMenu => {
                draw_overworld(&self.overworld, &mut self.resources, fb);
                draw_start_menu(&self.start_menu, &self.player_name, fb)
            }
            GameScreen::OptionsMenu => draw_options_menu(&self.options_menu, fb),
            GameScreen::SaveMenu => {
                draw_overworld(&self.overworld, &mut self.resources, fb);
                draw_save_menu(&self.save_menu, fb);
            }
        }
    }
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    if let Err(e) = console_log::init_with_level(log::Level::Info) {
        web_sys::console::error_1(&format!("Failed to initialize logger: {}", e).into());
    }
    
    if let Err(e) = run().await {
        web_sys::console::error_1(&format!("Game initialization failed: {}", e).into());
        
        let document = web_sys::window()
            .and_then(|w| w.document())
            .expect("no document");
        
let error_div = document.get_element_by_id("error")
            .expect("no error div");
        error_div.set_attribute("class", "").ok();
        
        let loading = document.get_element_by_id("loading")
            .expect("no loading div");
        loading.set_attribute("class", "hidden").ok();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    env_logger::init();
    if let Err(e) = pollster::block_on(run()) {
        eprintln!("Game initialization failed: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let version = GameVersion::Red;
    let event_loop = EventLoop::new()?;

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
                )?,
        )
    };

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        let game_canvas = window.canvas().unwrap();
        game_canvas.set_id("game-canvas");
        
        let old_canvas = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("game-canvas"))
            .expect("couldn't find canvas with id 'game-canvas'");
        
        let parent = old_canvas.parent_node().expect("canvas has no parent");
        parent
            .replace_child(&web_sys::Element::from(game_canvas), &old_canvas)
            .expect("couldn't replace canvas element");

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
            use pixels::wgpu::Backends;
            
            let texture_format = pixels::wgpu::TextureFormat::Rgba8Unorm;
            
            builder
                .texture_format(texture_format)
                .surface_texture_format(texture_format)
                // Some browsers expose partial WebGPU limits and return
                // `undefined` for numeric fields, which can panic in wasm-bindgen.
                // Keep wasm on WebGL for broad compatibility.
                .wgpu_backend(Backends::GL)
        };

        builder.build_async().await?
    };

    let mut game = PokemonGame::new(version);
    let mut frame_buffer = FrameBuffer::default();
    let mut input = InputState::new();

    // GB VBlank: 4194304 Hz / 70224 cycles ≈ 59.7275 Hz
    #[cfg(not(target_arch = "wasm32"))]
    const FRAME_DURATION: Duration = Duration::from_nanos(16_742_706);
    #[cfg(not(target_arch = "wasm32"))]
    let mut next_frame_time = Instant::now();

    let event_handler = move |event, elwt: &winit::event_loop::ActiveEventLoop| match event {
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
            #[cfg(target_arch = "wasm32")]
            {
                game.update(&input);
                input.begin_frame();
                window.request_redraw();
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let now = Instant::now();
                if now >= next_frame_time {
                    game.update(&input);
                    input.begin_frame();
                    window.request_redraw();
                    next_frame_time += FRAME_DURATION;
                    if next_frame_time < now {
                        next_frame_time = now + FRAME_DURATION;
                    }
                }
                let sleep_duration = next_frame_time.saturating_duration_since(Instant::now());
                if !sleep_duration.is_zero() {
                    std::thread::sleep(sleep_duration);
                }
            }
        }
        _ => {}
    };

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;

        #[allow(deprecated)]
        event_loop.spawn(event_handler);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let res = event_loop.run(event_handler);
        res?;
        Ok(())
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
