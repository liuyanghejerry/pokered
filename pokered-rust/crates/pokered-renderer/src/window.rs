use std::sync::Arc;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

use crate::input::InputState;
use crate::{FrameBuffer, DEFAULT_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct GameWindowConfig {
    pub title: String,
    pub scale: u32,
    pub resizable: bool,
}

impl Default for GameWindowConfig {
    fn default() -> Self {
        Self {
            title: "Pokémon Red/Blue".to_string(),
            scale: DEFAULT_SCALE,
            resizable: true,
        }
    }
}

pub trait GameLoop {
    fn update(&mut self, input: &InputState);
    fn draw(&mut self, frame_buffer: &mut FrameBuffer);
    fn should_exit(&self) -> bool {
        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Failed to create event loop: {0}")]
    EventLoop(String),
    #[error("Failed to create window: {0}")]
    WindowCreation(String),
    #[error("Failed to create pixel buffer: {0}")]
    PixelBuffer(#[from] pixels::Error),
}

pub fn run<G: GameLoop + 'static>(
    config: GameWindowConfig,
    mut game: G,
) -> Result<(), WindowError> {
    env_logger::init();
    let event_loop = EventLoop::new().map_err(|e| WindowError::EventLoop(e.to_string()))?;
    let window = {
        let size = LogicalSize::new(
            (SCREEN_WIDTH * config.scale) as f64,
            (SCREEN_HEIGHT * config.scale) as f64,
        );
        #[allow(deprecated)]
        Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&config.title)
                        .with_inner_size(size)
                        .with_min_inner_size(LogicalSize::new(
                            SCREEN_WIDTH as f64,
                            SCREEN_HEIGHT as f64,
                        ))
                        .with_resizable(config.resizable),
                )
                .map_err(|e| WindowError::WindowCreation(e.to_string()))?,
        )
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
    };

    let mut frame_buffer = FrameBuffer::default();
    let mut input = InputState::new();

    #[allow(deprecated)]
    let res = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
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
            if game.should_exit() {
                elwt.exit();
                return;
            }
            window.request_redraw();
        }
        _ => {}
    });

    res.map_err(|e| WindowError::EventLoop(e.to_string()))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
