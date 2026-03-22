use crate::palette::Palette;
use crate::tile::TileSet;
use crate::{FrameBuffer, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub const SCREEN_TILES_X: u32 = 20;
pub const SCREEN_TILES_Y: u32 = 18;
pub const SCREEN_TILE_COUNT: usize = (SCREEN_TILES_X * SCREEN_TILES_Y) as usize;

/// A 20×18 tile buffer representing the screen-space tilemap.
#[derive(Debug, Clone)]
pub struct ScreenTileBuffer {
    pub tiles: [u8; SCREEN_TILE_COUNT],
}

impl ScreenTileBuffer {
    pub fn new() -> Self {
        Self {
            tiles: [0x7F; SCREEN_TILE_COUNT],
        }
    }

    #[inline]
    pub fn get(&self, tx: u32, ty: u32) -> u8 {
        if tx >= SCREEN_TILES_X || ty >= SCREEN_TILES_Y {
            return 0x7F;
        }
        self.tiles[(ty * SCREEN_TILES_X + tx) as usize]
    }

    #[inline]
    pub fn set(&mut self, tx: u32, ty: u32, tile_id: u8) {
        if tx < SCREEN_TILES_X && ty < SCREEN_TILES_Y {
            self.tiles[(ty * SCREEN_TILES_X + tx) as usize] = tile_id;
        }
    }

    pub fn fill(&mut self, tile_id: u8) {
        self.tiles.fill(tile_id);
    }

    pub fn set_row(&mut self, ty: u32, row_data: &[u8]) {
        if ty >= SCREEN_TILES_Y {
            return;
        }
        let start = (ty * SCREEN_TILES_X) as usize;
        let count = row_data.len().min(SCREEN_TILES_X as usize);
        self.tiles[start..start + count].copy_from_slice(&row_data[..count]);
    }

    pub fn copy_from_flat(&mut self, data: &[u8]) {
        let count = data.len().min(SCREEN_TILE_COUNT);
        self.tiles[..count].copy_from_slice(&data[..count]);
    }

    pub fn render(&self, fb: &mut FrameBuffer, tileset: &TileSet, palette: &Palette) {
        for ty in 0..SCREEN_TILES_Y {
            for tx in 0..SCREEN_TILES_X {
                let tile_id = self.get(tx, ty) as usize;
                let tile = tileset.get(tile_id);
                let screen_x = tx * TILE_SIZE;
                let screen_y = ty * TILE_SIZE;

                for row in 0..TILE_SIZE {
                    for col in 0..TILE_SIZE {
                        let px = screen_x + col;
                        let py = screen_y + row;
                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            let color_idx = tile.get(row as usize, col as usize);
                            let rgba = palette.color(crate::palette::GbColor::from_u8(color_idx));
                            fb.set_pixel(px, py, rgba);
                        }
                    }
                }
            }
        }
    }

    pub fn render_region(
        &self,
        fb: &mut FrameBuffer,
        tileset: &TileSet,
        palette: &Palette,
        tx_start: u32,
        ty_start: u32,
        tw: u32,
        th: u32,
    ) {
        let tx_end = (tx_start + tw).min(SCREEN_TILES_X);
        let ty_end = (ty_start + th).min(SCREEN_TILES_Y);

        for ty in ty_start..ty_end {
            for tx in tx_start..tx_end {
                let tile_id = self.get(tx, ty) as usize;
                let tile = tileset.get(tile_id);
                let screen_x = tx * TILE_SIZE;
                let screen_y = ty * TILE_SIZE;

                for row in 0..TILE_SIZE {
                    for col in 0..TILE_SIZE {
                        let px = screen_x + col;
                        let py = screen_y + row;
                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            let color_idx = tile.get(row as usize, col as usize);
                            let rgba = palette.color(crate::palette::GbColor::from_u8(color_idx));
                            fb.set_pixel(px, py, rgba);
                        }
                    }
                }
            }
        }
    }
}

impl Default for ScreenTileBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn write_tiles_at(buf: &mut ScreenTileBuffer, start_tx: u32, start_ty: u32, tile_ids: &[u8]) {
    for (i, &tile_id) in tile_ids.iter().enumerate() {
        let tx = start_tx + i as u32;
        if tx >= SCREEN_TILES_X {
            break;
        }
        buf.set(tx, start_ty, tile_id);
    }
}
