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

use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, ScreenAction};
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::title_screen::TitleScreenState;
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: u32 = 3;

struct PokemonGame {
    state: GameState,
    title_screen: TitleScreenState,
    main_menu: MainMenuState,
    frame_count: u64,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(None);
        Self {
            state,
            title_screen,
            main_menu,
            frame_count: 0,
        }
    }

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
                    a: input.is_just_pressed(GbButton::A)
                        || input.is_just_pressed(GbButton::Start),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.main_menu.update_frame(menu_input)
            }
            GameScreen::OakSpeech
            | GameScreen::Overworld
            | GameScreen::Battle => ScreenAction::Continue,
        };

        if let ScreenAction::Transition(new_screen) = action {
            self.handle_transition(new_screen);
        }
    }

    fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => self.title_screen.reset(),
            GameScreen::MainMenu => {
                self.main_menu = MainMenuState::new(None);
            }
            GameScreen::CopyrightSplash => self.title_screen.reset(),
            _ => {}
        }
        self.state.transition_to(screen);
    }

    fn draw(&self, fb: &mut FrameBuffer) {
        fb.clear(Rgba::WHITE);
        match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, fb);
            }
            GameScreen::MainMenu => draw_main_menu(&self.main_menu, fb),
            GameScreen::OakSpeech => draw_placeholder("Oak Speech", fb),
            GameScreen::Overworld => draw_placeholder("Overworld", fb),
            GameScreen::Battle => draw_placeholder("Battle", fb),
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

fn draw_placeholder(name: &str, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    draw_text(name, 10, 60, Rgba::BLACK, fb);
    draw_text("Not yet implemented", 10, 80, Rgba::BLACK, fb);
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
            input.begin_frame();
            game.update(&input);
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
