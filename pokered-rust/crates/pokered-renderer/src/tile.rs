//! Tile decoding and tileset management.
//!
//! Game Boy tiles are 8×8 pixels, stored in 2bpp (2 bits per pixel) format.
//! Each row is 2 bytes: the low bit-plane and the high bit-plane.
//! Pixel colors are indices 0–3 into a palette.

use crate::palette::{GbColor, Palette};
use crate::Rgba;

/// Number of bytes per tile row in 2bpp format (low byte + high byte).
pub const BYTES_PER_TILE_ROW: usize = 2;
/// Total bytes per 8×8 tile in 2bpp format.
pub const BYTES_PER_TILE: usize = 16;
/// Number of pixels per tile side.
pub const TILE_PIXELS: usize = 8;

/// A decoded 8×8 tile. Each element is a color index (0–3).
#[derive(Debug, Clone)]
pub struct Tile {
    /// 8 rows × 8 columns of palette indices (0–3).
    /// Indexed as `pixels[row][col]`.
    pub pixels: [[u8; TILE_PIXELS]; TILE_PIXELS],
}

impl Tile {
    /// Decode a tile from 16 bytes of 2bpp data.
    ///
    /// Each row is 2 bytes: `low_byte` then `high_byte`.
    /// Bit 7 = leftmost pixel. The color index for pixel `x` is:
    ///   `((high_byte >> (7-x)) & 1) << 1 | ((low_byte >> (7-x)) & 1)`
    pub fn from_2bpp(data: &[u8]) -> Self {
        assert!(
            data.len() >= BYTES_PER_TILE,
            "Need {} bytes for a tile, got {}",
            BYTES_PER_TILE,
            data.len()
        );
        let mut pixels = [[0u8; TILE_PIXELS]; TILE_PIXELS];
        for row in 0..TILE_PIXELS {
            let lo = data[row * 2];
            let hi = data[row * 2 + 1];
            for col in 0..TILE_PIXELS {
                let bit = 7 - col;
                let color_index = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
                pixels[row][col] = color_index;
            }
        }
        Self { pixels }
    }

    /// Create a blank (all color 0) tile.
    pub fn blank() -> Self {
        Self {
            pixels: [[0; TILE_PIXELS]; TILE_PIXELS],
        }
    }

    /// Get the color index at (row, col).
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.pixels[row][col]
    }

    /// Render this tile's row into RGBA pixels using a palette.
    /// Returns 8 RGBA values for the given tile row.
    pub fn render_row(&self, row: usize, palette: &Palette) -> [Rgba; TILE_PIXELS] {
        let mut out = [Rgba::TRANSPARENT; TILE_PIXELS];
        for col in 0..TILE_PIXELS {
            let color_idx = GbColor::from_u8(self.pixels[row][col]);
            out[col] = palette.color(color_idx);
        }
        out
    }

    /// Check if this tile is vertically flipped.
    pub fn flip_y(&self) -> Tile {
        let mut pixels = [[0u8; TILE_PIXELS]; TILE_PIXELS];
        for row in 0..TILE_PIXELS {
            pixels[row] = self.pixels[TILE_PIXELS - 1 - row];
        }
        Tile { pixels }
    }

    /// Check if this tile is horizontally flipped.
    pub fn flip_x(&self) -> Tile {
        let mut pixels = [[0u8; TILE_PIXELS]; TILE_PIXELS];
        for row in 0..TILE_PIXELS {
            for col in 0..TILE_PIXELS {
                pixels[row][col] = self.pixels[row][TILE_PIXELS - 1 - col];
            }
        }
        Tile { pixels }
    }
}

/// A set of decoded tiles, indexed by tile number.
#[derive(Debug, Clone)]
pub struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    /// Create a tileset by decoding 2bpp tile data.
    /// The data length must be a multiple of 16 (bytes per tile).
    pub fn from_2bpp(data: &[u8]) -> Self {
        assert!(
            data.len() % BYTES_PER_TILE == 0,
            "Tile data length {} is not a multiple of {}",
            data.len(),
            BYTES_PER_TILE,
        );
        let count = data.len() / BYTES_PER_TILE;
        let mut tiles = Vec::with_capacity(count);
        for i in 0..count {
            let start = i * BYTES_PER_TILE;
            tiles.push(Tile::from_2bpp(&data[start..start + BYTES_PER_TILE]));
        }
        Self { tiles }
    }

    /// Create an empty tileset with `count` blank tiles.
    pub fn blank(count: usize) -> Self {
        Self {
            tiles: vec![Tile::blank(); count],
        }
    }

    /// Get a tile by index. Returns blank tile if out of bounds.
    pub fn get(&self, index: usize) -> &Tile {
        if index < self.tiles.len() {
            &self.tiles[index]
        } else {
            // Return a static blank tile for out-of-bounds access
            // This matches GB behavior where VRAM reads wrap
            static BLANK: Tile = Tile {
                pixels: [[0; TILE_PIXELS]; TILE_PIXELS],
            };
            &BLANK
        }
    }

    /// Number of tiles in this set.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Whether this tileset is empty.
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Replace a tile at the given index.
    pub fn set(&mut self, index: usize, tile: Tile) {
        if index < self.tiles.len() {
            self.tiles[index] = tile;
        }
    }

    /// Load raw 2bpp data into the tileset starting at tile index `start_tile`.
    /// Overwrites existing tiles.
    pub fn load_2bpp_at(&mut self, start_tile: usize, data: &[u8]) {
        let count = data.len() / BYTES_PER_TILE;
        for i in 0..count {
            let tile_idx = start_tile + i;
            if tile_idx >= self.tiles.len() {
                break;
            }
            let start = i * BYTES_PER_TILE;
            self.tiles[tile_idx] = Tile::from_2bpp(&data[start..start + BYTES_PER_TILE]);
        }
    }

    /// Decode 1bpp tile data (8 bytes per tile, 1 bit per pixel).
    /// Color 0 → palette index 0, color 1 → palette index 3 (black).
    pub fn from_1bpp(data: &[u8]) -> Self {
        let bytes_per_tile_1bpp = 8;
        assert!(
            data.len() % bytes_per_tile_1bpp == 0,
            "1bpp tile data length {} is not a multiple of {}",
            data.len(),
            bytes_per_tile_1bpp,
        );
        let count = data.len() / bytes_per_tile_1bpp;
        let mut tiles = Vec::with_capacity(count);
        for i in 0..count {
            let mut pixels = [[0u8; TILE_PIXELS]; TILE_PIXELS];
            for row in 0..TILE_PIXELS {
                let byte = data[i * bytes_per_tile_1bpp + row];
                for col in 0..TILE_PIXELS {
                    let bit = 7 - col;
                    // 1bpp: bit=1 → color 3 (black), bit=0 → color 0 (white)
                    pixels[row][col] = if (byte >> bit) & 1 == 1 { 3 } else { 0 };
                }
            }
            tiles.push(Tile { pixels });
        }
        Self { tiles }
    }
}

/// Decode a single 2bpp tile row (2 bytes) into 8 color indices.
pub fn decode_2bpp_row(lo: u8, hi: u8) -> [u8; TILE_PIXELS] {
    let mut out = [0u8; TILE_PIXELS];
    for col in 0..TILE_PIXELS {
        let bit = 7 - col;
        out[col] = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
    }
    out
}
