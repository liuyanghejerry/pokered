//! Pokémon Red/Blue - Web/WASM Build
//!
//! This crate provides a WebAssembly-compatible build of the game,
//! using pixels + winit with async initialization for wasm32 targets.
//! Also runs natively as a fallback for development.
//!
//! The game logic is shared with pokered-app crate via PokemonGame,
//! ensuring identical behavior between web and native builds.

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

use pokered_app::PokemonGame;
use pokered_core::data::wild_data::GameVersion;
use pokered_renderer::input::InputState;
use pokered_renderer::{FrameBuffer, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: u32 = 3;

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

        let error_div = document.get_element_by_id("error").expect("no error div");
        error_div.set_attribute("class", "").ok();

        let loading = document
            .get_element_by_id("loading")
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
            event_loop.create_window(
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
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let _ = resize_window.request_inner_size(get_window_size());
        }) as Box<dyn FnMut(_)>);
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
