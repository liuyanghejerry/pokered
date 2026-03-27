mod battle;
mod menu;
mod oak;
mod overworld;
mod title;

pub use battle::draw_battle;
pub use menu::{draw_main_menu, draw_options_menu, draw_save_menu, draw_start_menu};
pub use oak::draw_oak_speech;
pub use overworld::draw_overworld;
pub use title::draw_title_screen;

use pokered_renderer::embedded_font::{box_tiles, draw_glyph, fill_tile};
use pokered_renderer::palette::Palette;
use pokered_renderer::tile::TileSet;
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub fn blit_tileset(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    x: u32,
    y: u32,
    tiles_per_row: u32,
    palette: &Palette,
) {
    let total = tileset.len();
    for idx in 0..total {
        let tile = tileset.get(idx);
        let tx = (idx as u32) % tiles_per_row;
        let ty = (idx as u32) / tiles_per_row;
        let px = x + tx * TILE_SIZE;
        let py = y + ty * TILE_SIZE;
        for row in 0..TILE_SIZE {
            let rgba_row = tile.render_row(row as usize, palette);
            for col in 0..TILE_SIZE {
                let sx = px + col;
                let sy = py + row;
                if sx < SCREEN_WIDTH && sy < SCREEN_HEIGHT {
                    let c = rgba_row[col as usize];
                    if c != Rgba::TRANSPARENT {
                        fb.set_pixel(sx, sy, c);
                    }
                }
            }
        }
    }
}

pub fn draw_text_box(fb: &mut FrameBuffer, bx: u32, by: u32, bw: u32, bh: u32, color: Rgba) {
    let bg = Rgba::WHITE;
    let t = TILE_SIZE;

    draw_glyph(&box_tiles::TOP_LEFT, bx, by, color, bg, fb);
    for col in 0..bw {
        draw_glyph(
            &box_tiles::HORIZONTAL,
            bx + (1 + col) * t,
            by,
            color,
            bg,
            fb,
        );
    }
    draw_glyph(&box_tiles::TOP_RIGHT, bx + (1 + bw) * t, by, color, bg, fb);

    for row in 0..bh {
        let y = by + (1 + row) * t;
        draw_glyph(&box_tiles::VERTICAL_LEFT, bx, y, color, bg, fb);
        for col in 0..bw {
            fill_tile(bx + (1 + col) * t, y, bg, fb);
        }
        draw_glyph(
            &box_tiles::VERTICAL_RIGHT,
            bx + (1 + bw) * t,
            y,
            color,
            bg,
            fb,
        );
    }

    let bot_y = by + (1 + bh) * t;
    draw_glyph(&box_tiles::BOTTOM_LEFT, bx, bot_y, color, bg, fb);
    for col in 0..bw {
        draw_glyph(
            &box_tiles::HORIZONTAL_BOTTOM,
            bx + (1 + col) * t,
            bot_y,
            color,
            bg,
            fb,
        );
    }
    draw_glyph(
        &box_tiles::BOTTOM_RIGHT,
        bx + (1 + bw) * t,
        bot_y,
        color,
        bg,
        fb,
    );
}

pub fn draw_centered_sprite(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    sprite_w: u32,
    _sprite_h: u32,
    pal: &Palette,
) {
    let tiles_per_row = sprite_w / TILE_SIZE;
    let sx = (SCREEN_WIDTH.saturating_sub(sprite_w)) / 2;
    let sy = 32_u32;
    blit_tileset(fb, tileset, sx, sy, tiles_per_row, pal);
}

pub fn blit_single_tile(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    tile_idx: usize,
    px: u32,
    py: u32,
    palette: &Palette,
) {
    if tile_idx >= tileset.len() {
        return;
    }
    let tile = tileset.get(tile_idx);
    for row in 0..TILE_SIZE {
        let rgba_row = tile.render_row(row as usize, palette);
        for col in 0..TILE_SIZE {
            let sx = px + col;
            let sy = py + row;
            if sx < SCREEN_WIDTH && sy < SCREEN_HEIGHT {
                let c = rgba_row[col as usize];
                if c != Rgba::TRANSPARENT {
                    fb.set_pixel(sx, sy, c);
                }
            }
        }
    }
}

pub fn species_to_sprite_name(species_display: &str) -> String {
    species_display.to_lowercase().replace([' ', '-', '\''], "")
}

fn hp_bar_color(hp: u16, max_hp: u16) -> Rgba {
    if max_hp == 0 {
        return Rgba::BLACK;
    }
    let ratio = hp as f32 / max_hp as f32;
    if ratio > 0.5 {
        Rgba::rgb(0x55, 0xAA, 0x55)
    } else if ratio > 0.2 {
        Rgba::rgb(0xAA, 0xAA, 0x55)
    } else {
        Rgba::rgb(0xAA, 0x55, 0x55)
    }
}

fn draw_hp_bar(fb: &mut FrameBuffer, x: u32, y: u32, width: u32, hp: u16, max_hp: u16) {
    if max_hp == 0 {
        return;
    }
    let color = hp_bar_color(hp, max_hp);
    let filled = if hp > 0 {
        ((hp as f32 / max_hp as f32) * width as f32).ceil() as u32
    } else {
        0
    };

    for i in 0..width {
        let px = x + i;
        let c = if i < filled { color } else { Rgba::WHITE };
        for py in y..y + 8 {
            if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                fb.set_pixel(px, py, c);
            }
        }
    }
}
