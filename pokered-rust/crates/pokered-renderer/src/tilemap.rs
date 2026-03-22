//! Tile map — a 32×32 grid of tile indices that forms the background layer.
//!
//! The Game Boy has a 256×256 pixel background composed of a 32×32 tile map.
//! Each entry is a single byte (tile index into the current tile set).
//! The visible 160×144 screen is a scrollable viewport over this map.

use crate::palette::Palette;
use crate::tile::TileSet;
use crate::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

/// Game Boy background map dimensions in tiles.
pub const BG_MAP_WIDTH: u32 = 32;
pub const BG_MAP_HEIGHT: u32 = 32;
/// Background map size in pixels (256×256).
pub const BG_MAP_PIXEL_WIDTH: u32 = BG_MAP_WIDTH * TILE_SIZE;
pub const BG_MAP_PIXEL_HEIGHT: u32 = BG_MAP_HEIGHT * TILE_SIZE;
/// Total tile entries in one background map (1024).
pub const BG_MAP_SIZE: usize = (BG_MAP_WIDTH * BG_MAP_HEIGHT) as usize;

/// A 32×32 tile map representing the Game Boy background.
#[derive(Debug, Clone)]
pub struct TileMap {
    /// Tile indices, row-major. `indices[row * 32 + col]` = tile number.
    pub indices: Vec<u8>,
}

impl TileMap {
    /// Create a new tile map with all entries set to tile 0.
    pub fn new() -> Self {
        Self {
            indices: vec![0u8; BG_MAP_SIZE],
        }
    }

    /// Create a tile map from raw data (must be exactly 1024 bytes).
    pub fn from_data(data: &[u8]) -> Self {
        assert!(
            data.len() >= BG_MAP_SIZE,
            "TileMap needs {} bytes, got {}",
            BG_MAP_SIZE,
            data.len()
        );
        Self {
            indices: data[..BG_MAP_SIZE].to_vec(),
        }
    }

    /// Get the tile index at (tile_x, tile_y). Wraps around at 32.
    #[inline]
    pub fn get(&self, tile_x: u32, tile_y: u32) -> u8 {
        let x = (tile_x % BG_MAP_WIDTH) as usize;
        let y = (tile_y % BG_MAP_HEIGHT) as usize;
        self.indices[y * BG_MAP_WIDTH as usize + x]
    }

    /// Set the tile index at (tile_x, tile_y). Wraps around at 32.
    #[inline]
    pub fn set(&mut self, tile_x: u32, tile_y: u32, tile_index: u8) {
        let x = (tile_x % BG_MAP_WIDTH) as usize;
        let y = (tile_y % BG_MAP_HEIGHT) as usize;
        self.indices[y * BG_MAP_WIDTH as usize + x] = tile_index;
    }

    /// Fill a rectangular region of the tile map with a single tile index.
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, tile_index: u8) {
        for row in y..y + height {
            for col in x..x + width {
                self.set(col, row, tile_index);
            }
        }
    }

    /// Copy a block of tile indices into the map at position (dst_x, dst_y).
    /// `src` is row-major with the given `src_width`.
    pub fn copy_block(&mut self, dst_x: u32, dst_y: u32, src: &[u8], src_width: u32) {
        if src_width == 0 {
            return;
        }
        let src_height = src.len() as u32 / src_width;
        for row in 0..src_height {
            for col in 0..src_width {
                let idx = (row * src_width + col) as usize;
                if idx < src.len() {
                    self.set(dst_x + col, dst_y + row, src[idx]);
                }
            }
        }
    }

    /// Render the background into a framebuffer with scroll offset.
    ///
    /// This renders the 160×144 viewport of the 256×256 bg map,
    /// wrapping around at the edges (as the real Game Boy does).
    pub fn render(
        &self,
        fb: &mut FrameBuffer,
        tileset: &TileSet,
        palette: &Palette,
        scroll_x: u32,
        scroll_y: u32,
    ) {
        for screen_y in 0..SCREEN_HEIGHT {
            self.render_scanline(fb, tileset, palette, scroll_x, scroll_y, screen_y);
        }
    }

    /// Render a single scanline of the background.
    pub fn render_scanline(
        &self,
        fb: &mut FrameBuffer,
        tileset: &TileSet,
        palette: &Palette,
        scroll_x: u32,
        scroll_y: u32,
        screen_y: u32,
    ) {
        let bg_y = (scroll_y + screen_y) % BG_MAP_PIXEL_HEIGHT;
        let tile_row = bg_y / TILE_SIZE;
        let pixel_row = (bg_y % TILE_SIZE) as usize;

        for screen_x in 0..SCREEN_WIDTH {
            let bg_x = (scroll_x + screen_x) % BG_MAP_PIXEL_WIDTH;
            let tile_col = bg_x / TILE_SIZE;
            let pixel_col = (bg_x % TILE_SIZE) as usize;

            let tile_index = self.get(tile_col, tile_row) as usize;
            let tile = tileset.get(tile_index);
            let color_idx = tile.get(pixel_row, pixel_col);

            let rgba = palette.color(crate::palette::GbColor::from_u8(color_idx));
            fb.set_pixel(screen_x, screen_y, rgba);
        }
    }

    /// Render only the tiles within the given pixel rectangle.
    /// Useful for partial screen updates.
    pub fn render_rect(
        &self,
        fb: &mut FrameBuffer,
        tileset: &TileSet,
        palette: &Palette,
        scroll_x: u32,
        scroll_y: u32,
        rect_x: u32,
        rect_y: u32,
        rect_w: u32,
        rect_h: u32,
    ) {
        let x_end = (rect_x + rect_w).min(SCREEN_WIDTH);
        let y_end = (rect_y + rect_h).min(SCREEN_HEIGHT);

        for screen_y in rect_y..y_end {
            let bg_y = (scroll_y + screen_y) % BG_MAP_PIXEL_HEIGHT;
            let tile_row = bg_y / TILE_SIZE;
            let pixel_row = (bg_y % TILE_SIZE) as usize;

            for screen_x in rect_x..x_end {
                let bg_x = (scroll_x + screen_x) % BG_MAP_PIXEL_WIDTH;
                let tile_col = bg_x / TILE_SIZE;
                let pixel_col = (bg_x % TILE_SIZE) as usize;

                let tile_index = self.get(tile_col, tile_row) as usize;
                let tile = tileset.get(tile_index);
                let color_idx = tile.get(pixel_row, pixel_col);

                let rgba = palette.color(crate::palette::GbColor::from_u8(color_idx));
                fb.set_pixel(screen_x, screen_y, rgba);
            }
        }
    }
}

impl Default for TileMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Render state combining tilemap + tileset + palette + scroll for the BG layer.
#[derive(Debug, Clone)]
pub struct BgLayer {
    pub tilemap: TileMap,
    pub scroll_x: u32,
    pub scroll_y: u32,
}

impl BgLayer {
    pub fn new() -> Self {
        Self {
            tilemap: TileMap::new(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    /// Render the full background layer to the framebuffer.
    pub fn render(&self, fb: &mut FrameBuffer, tileset: &TileSet, palette: &Palette) {
        self.tilemap
            .render(fb, tileset, palette, self.scroll_x, self.scroll_y);
    }
}

impl Default for BgLayer {
    fn default() -> Self {
        Self::new()
    }
}
