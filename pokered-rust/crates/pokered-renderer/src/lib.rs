// pokered-renderer: Graphics rendering layer for Pokémon Red/Blue Rust rewrite
//
// This is NOT a Game Boy hardware emulator. It provides a higher-level
// rendering API that draws into a 160×144 pixel framebuffer and displays
// it via a scaled window using the `pixels` crate.

pub mod battle_anim;
pub mod battle_scene;
pub mod embedded_font;
pub mod input;
pub mod menu;
pub mod palette;
pub mod resource;
pub mod sprite;
pub mod text_renderer;
pub mod textbox;
pub mod tile;
pub mod tilemap;
pub mod transition;
pub mod viewport;
#[cfg(not(target_arch = "wasm32"))]
pub mod window;
pub mod window_layer;

/// Game Boy screen width in pixels
pub const SCREEN_WIDTH: u32 = 160;
/// Game Boy screen height in pixels
pub const SCREEN_HEIGHT: u32 = 144;
/// Default integer scale factor for the window
pub const DEFAULT_SCALE: u32 = 3;
/// Game Boy tile size in pixels (8×8)
pub const TILE_SIZE: u32 = 8;
/// Screen width in tiles (160 / 8 = 20)
pub const SCREEN_WIDTH_TILES: u32 = SCREEN_WIDTH / TILE_SIZE;
/// Screen height in tiles (144 / 8 = 18)
pub const SCREEN_HEIGHT_TILES: u32 = SCREEN_HEIGHT / TILE_SIZE;
/// Bytes per pixel in the RGBA framebuffer
pub const BYTES_PER_PIXEL: usize = 4;
/// Total framebuffer size in bytes (160 * 144 * 4)
pub const FRAMEBUFFER_SIZE: usize =
    (SCREEN_WIDTH as usize) * (SCREEN_HEIGHT as usize) * BYTES_PER_PIXEL;

/// An RGBA color (red, green, blue, alpha).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 0xFF])
    }

    pub const WHITE: Self = Self::rgb(0xFF, 0xFF, 0xFF);
    pub const BLACK: Self = Self::rgb(0x00, 0x00, 0x00);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

/// A 160×144 RGBA pixel framebuffer.
///
/// The internal buffer is a flat array of RGBA bytes in row-major order.
/// Pixel (x, y) starts at byte offset `(y * 160 + x) * 4`.
pub struct FrameBuffer {
    /// Raw RGBA pixel data, 160*144*4 = 92160 bytes
    pub data: Vec<u8>,
}

impl FrameBuffer {
    /// Create a new framebuffer, cleared to the given color.
    pub fn new(clear_color: Rgba) -> Self {
        let mut fb = Self {
            data: vec![0; FRAMEBUFFER_SIZE],
        };
        fb.clear(clear_color);
        fb
    }

    /// Clear the entire framebuffer to a single color.
    pub fn clear(&mut self, color: Rgba) {
        for pixel in self.data.chunks_exact_mut(BYTES_PER_PIXEL) {
            pixel.copy_from_slice(&color.0);
        }
    }

    /// Set a single pixel. Returns false if out of bounds.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba) -> bool {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return false;
        }
        let offset = ((y as usize) * (SCREEN_WIDTH as usize) + (x as usize)) * BYTES_PER_PIXEL;
        self.data[offset..offset + BYTES_PER_PIXEL].copy_from_slice(&color.0);
        true
    }

    /// Get the color of a single pixel. Returns None if out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba> {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return None;
        }
        let offset = ((y as usize) * (SCREEN_WIDTH as usize) + (x as usize)) * BYTES_PER_PIXEL;
        let mut c = [0u8; 4];
        c.copy_from_slice(&self.data[offset..offset + BYTES_PER_PIXEL]);
        Some(Rgba(c))
    }

    /// Fill a rectangular region with a color. Coordinates are clamped to screen bounds.
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Rgba) {
        let x_start = x.min(SCREEN_WIDTH);
        let y_start = y.min(SCREEN_HEIGHT);
        let x_end = (x + width).min(SCREEN_WIDTH);
        let y_end = (y + height).min(SCREEN_HEIGHT);

        for row in y_start..y_end {
            let row_offset = (row as usize) * (SCREEN_WIDTH as usize) * BYTES_PER_PIXEL;
            for col in x_start..x_end {
                let offset = row_offset + (col as usize) * BYTES_PER_PIXEL;
                self.data[offset..offset + BYTES_PER_PIXEL].copy_from_slice(&color.0);
            }
        }
    }

    /// Get a slice of one pixel row's RGBA data. Returns None if y is out of bounds.
    pub fn row_slice(&self, y: u32) -> Option<&[u8]> {
        if y >= SCREEN_HEIGHT {
            return None;
        }
        let start = (y as usize) * (SCREEN_WIDTH as usize) * BYTES_PER_PIXEL;
        let end = start + (SCREEN_WIDTH as usize) * BYTES_PER_PIXEL;
        Some(&self.data[start..end])
    }

    /// Get a mutable slice of one pixel row's RGBA data. Returns None if y is out of bounds.
    pub fn row_slice_mut(&mut self, y: u32) -> Option<&mut [u8]> {
        if y >= SCREEN_HEIGHT {
            return None;
        }
        let start = (y as usize) * (SCREEN_WIDTH as usize) * BYTES_PER_PIXEL;
        let end = start + (SCREEN_WIDTH as usize) * BYTES_PER_PIXEL;
        Some(&mut self.data[start..end])
    }

    /// Copy a horizontal line of RGBA data into the framebuffer.
    /// `src` must be exactly `count * 4` bytes.
    /// Returns false if the line goes out of bounds.
    pub fn blit_row(&mut self, x: u32, y: u32, src: &[u8], count: u32) -> bool {
        if y >= SCREEN_HEIGHT || x >= SCREEN_WIDTH {
            return false;
        }
        let actual_count = count.min(SCREEN_WIDTH - x) as usize;
        let src_bytes = actual_count * BYTES_PER_PIXEL;
        if src.len() < src_bytes {
            return false;
        }
        let offset = ((y as usize) * (SCREEN_WIDTH as usize) + (x as usize)) * BYTES_PER_PIXEL;
        self.data[offset..offset + src_bytes].copy_from_slice(&src[..src_bytes]);
        true
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new(Rgba::WHITE)
    }
}

#[cfg(test)]
mod tests;
