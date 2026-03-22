use super::*;
use crate::input::{keycode_to_gb_button, GbButton, InputState};
use crate::palette::*;
use winit::keyboard::KeyCode;

#[test]
fn constants_are_consistent() {
    assert_eq!(SCREEN_WIDTH, 160);
    assert_eq!(SCREEN_HEIGHT, 144);
    assert_eq!(TILE_SIZE, 8);
    assert_eq!(SCREEN_WIDTH_TILES, 20);
    assert_eq!(SCREEN_HEIGHT_TILES, 18);
    assert_eq!(BYTES_PER_PIXEL, 4);
    assert_eq!(FRAMEBUFFER_SIZE, 160 * 144 * 4);
}

#[test]
fn rgba_constructors() {
    let c = Rgba::new(1, 2, 3, 4);
    assert_eq!(c.0, [1, 2, 3, 4]);
    let c2 = Rgba::rgb(10, 20, 30);
    assert_eq!(c2.0, [10, 20, 30, 0xFF]);
    assert_eq!(Rgba::WHITE.0, [0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(Rgba::BLACK.0, [0, 0, 0, 0xFF]);
    assert_eq!(Rgba::TRANSPARENT.0, [0, 0, 0, 0]);
}

#[test]
fn framebuffer_new_cleared() {
    let fb = FrameBuffer::new(Rgba::BLACK);
    assert_eq!(fb.data.len(), FRAMEBUFFER_SIZE);
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(159, 143), Some(Rgba::BLACK));
}

#[test]
fn framebuffer_default_is_white() {
    let fb = FrameBuffer::default();
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE));
}

#[test]
fn framebuffer_set_get_pixel() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let red = Rgba::rgb(0xFF, 0, 0);
    assert!(fb.set_pixel(10, 20, red));
    assert_eq!(fb.get_pixel(10, 20), Some(red));
    assert_eq!(fb.get_pixel(11, 20), Some(Rgba::WHITE));
}

#[test]
fn framebuffer_out_of_bounds() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    assert!(!fb.set_pixel(160, 0, Rgba::BLACK));
    assert!(!fb.set_pixel(0, 144, Rgba::BLACK));
    assert!(!fb.set_pixel(200, 200, Rgba::BLACK));
    assert_eq!(fb.get_pixel(160, 0), None);
    assert_eq!(fb.get_pixel(0, 144), None);
}

#[test]
fn framebuffer_clear() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    fb.set_pixel(50, 50, Rgba::BLACK);
    fb.clear(Rgba::rgb(0x12, 0x34, 0x56));
    assert_eq!(fb.get_pixel(50, 50), Some(Rgba::rgb(0x12, 0x34, 0x56)));
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::rgb(0x12, 0x34, 0x56)));
}

#[test]
fn framebuffer_fill_rect() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let blue = Rgba::rgb(0, 0, 0xFF);
    fb.fill_rect(10, 20, 5, 3, blue);
    for y in 20..23 {
        for x in 10..15 {
            assert_eq!(fb.get_pixel(x, y), Some(blue), "pixel ({x}, {y})");
        }
    }
    assert_eq!(fb.get_pixel(9, 20), Some(Rgba::WHITE));
    assert_eq!(fb.get_pixel(15, 20), Some(Rgba::WHITE));
    assert_eq!(fb.get_pixel(10, 19), Some(Rgba::WHITE));
    assert_eq!(fb.get_pixel(10, 23), Some(Rgba::WHITE));
}

#[test]
fn framebuffer_fill_rect_clamped() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    fb.fill_rect(155, 140, 20, 20, Rgba::BLACK);
    assert_eq!(fb.get_pixel(155, 140), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(159, 143), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(154, 140), Some(Rgba::WHITE));
}

#[test]
fn framebuffer_row_slice() {
    let fb = FrameBuffer::new(Rgba::WHITE);
    let row = fb.row_slice(0).unwrap();
    assert_eq!(row.len(), 160 * 4);
    assert_eq!(&row[0..4], &[0xFF, 0xFF, 0xFF, 0xFF]);
    assert!(fb.row_slice(144).is_none());
}

#[test]
fn framebuffer_row_slice_mut() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    {
        let row = fb.row_slice_mut(5).unwrap();
        row[0] = 0x12;
        row[1] = 0x34;
        row[2] = 0x56;
        row[3] = 0x78;
    }
    assert_eq!(fb.get_pixel(0, 5), Some(Rgba::new(0x12, 0x34, 0x56, 0x78)));
}

#[test]
fn framebuffer_blit_row() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let src = [0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44];
    assert!(fb.blit_row(5, 10, &src, 2));
    assert_eq!(fb.get_pixel(5, 10), Some(Rgba::new(0xAA, 0xBB, 0xCC, 0xDD)));
    assert_eq!(fb.get_pixel(6, 10), Some(Rgba::new(0x11, 0x22, 0x33, 0x44)));
    assert_eq!(fb.get_pixel(4, 10), Some(Rgba::WHITE));
}

#[test]
fn framebuffer_blit_row_out_of_bounds() {
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let src = [0; 8];
    assert!(!fb.blit_row(160, 0, &src, 2));
    assert!(!fb.blit_row(0, 144, &src, 2));
}

// --- Palette tests ---

#[test]
fn gb_color_from_u8() {
    assert_eq!(GbColor::from_u8(0), GbColor::White);
    assert_eq!(GbColor::from_u8(1), GbColor::LightGray);
    assert_eq!(GbColor::from_u8(2), GbColor::DarkGray);
    assert_eq!(GbColor::from_u8(3), GbColor::Black);
    assert_eq!(GbColor::from_u8(4), GbColor::White);
    assert_eq!(GbColor::from_u8(0xFF), GbColor::Black);
}

#[test]
fn dmg_palette_colors() {
    assert_eq!(
        DMG_PALETTE.color(GbColor::White),
        Rgba::rgb(0x9B, 0xBC, 0x0F)
    );
    assert_eq!(
        DMG_PALETTE.color(GbColor::LightGray),
        Rgba::rgb(0x8B, 0xAC, 0x0F)
    );
    assert_eq!(
        DMG_PALETTE.color(GbColor::DarkGray),
        Rgba::rgb(0x30, 0x62, 0x30)
    );
    assert_eq!(
        DMG_PALETTE.color(GbColor::Black),
        Rgba::rgb(0x0F, 0x38, 0x0F)
    );
}

#[test]
fn grayscale_palette_colors() {
    assert_eq!(
        GRAYSCALE_PALETTE.color(GbColor::White),
        Rgba::rgb(0xFF, 0xFF, 0xFF)
    );
    assert_eq!(GRAYSCALE_PALETTE.color(GbColor::Black), Rgba::rgb(0, 0, 0));
}

#[test]
fn bgp_register_normal() {
    let pal = Palette::from_bgp_register(DEFAULT_BGP, &GRAYSCALE_PALETTE);
    assert_eq!(pal.colors[0], Rgba::rgb(0xFF, 0xFF, 0xFF));
    assert_eq!(pal.colors[1], Rgba::rgb(0xAA, 0xAA, 0xAA));
    assert_eq!(pal.colors[2], Rgba::rgb(0x55, 0x55, 0x55));
    assert_eq!(pal.colors[3], Rgba::rgb(0x00, 0x00, 0x00));
}

#[test]
fn bgp_register_inverted() {
    let inverted: u8 = 0b00011011;
    let pal = Palette::from_bgp_register(inverted, &GRAYSCALE_PALETTE);
    assert_eq!(pal.colors[0], Rgba::rgb(0x00, 0x00, 0x00));
    assert_eq!(pal.colors[1], Rgba::rgb(0x55, 0x55, 0x55));
    assert_eq!(pal.colors[2], Rgba::rgb(0xAA, 0xAA, 0xAA));
    assert_eq!(pal.colors[3], Rgba::rgb(0xFF, 0xFF, 0xFF));
}

#[test]
fn bgp_register_white_out() {
    let pal = Palette::from_bgp_register(0x00, &GRAYSCALE_PALETTE);
    for i in 0..4 {
        assert_eq!(pal.colors[i], Rgba::rgb(0xFF, 0xFF, 0xFF));
    }
}

#[test]
fn palette_state_default() {
    let ps = PaletteState::default();
    assert_eq!(ps.bgp, DEFAULT_BGP);
    assert_eq!(ps.obp0, DEFAULT_OBP0);
    assert_eq!(ps.obp1, DEFAULT_OBP1);
}

#[test]
fn palette_state_white_out_and_reset() {
    let mut ps = PaletteState::default();
    ps.white_out();
    assert_eq!(ps.bgp, 0);
    assert_eq!(ps.obp0, 0);
    assert_eq!(ps.obp1, 0);
    ps.reset_normal();
    assert_eq!(ps.bgp, DEFAULT_BGP);
    assert_eq!(ps.obp0, DEFAULT_OBP0);
}

// --- Input tests ---

#[test]
fn input_state_initial() {
    let input = InputState::new();
    for button in GbButton::ALL {
        assert!(!input.is_held(button));
        assert!(!input.is_just_pressed(button));
        assert!(!input.is_just_released(button));
    }
    assert!(!input.any_held());
    assert!(!input.any_just_pressed());
}

#[test]
fn input_press_and_release() {
    let mut input = InputState::new();
    input.press(GbButton::A);
    assert!(input.is_held(GbButton::A));
    assert!(!input.is_held(GbButton::B));
    input.release(GbButton::A);
    assert!(!input.is_held(GbButton::A));
}

#[test]
fn input_just_pressed() {
    let mut input = InputState::new();
    input.press(GbButton::Start);
    assert!(input.is_just_pressed(GbButton::Start));
    input.begin_frame();
    assert!(!input.is_just_pressed(GbButton::Start));
    assert!(input.is_held(GbButton::Start));
}

#[test]
fn input_just_released() {
    let mut input = InputState::new();
    input.press(GbButton::Up);
    input.begin_frame();
    input.release(GbButton::Up);
    assert!(input.is_just_released(GbButton::Up));
}

#[test]
fn input_any_flags() {
    let mut input = InputState::new();
    assert!(!input.any_held());
    input.press(GbButton::Down);
    assert!(input.any_held());
    assert!(input.any_just_pressed());
}

#[test]
fn input_clear() {
    let mut input = InputState::new();
    input.press(GbButton::A);
    input.press(GbButton::B);
    input.clear();
    assert!(!input.any_held());
    assert_eq!(input.raw_current(), 0);
    assert_eq!(input.raw_previous(), 0);
}

#[test]
fn input_raw_values() {
    let mut input = InputState::new();
    input.press(GbButton::A);
    input.press(GbButton::Start);
    assert_eq!(
        input.raw_current(),
        GbButton::A.bit_mask() | GbButton::Start.bit_mask()
    );
}

#[test]
fn input_keycode_mapping() {
    assert_eq!(keycode_to_gb_button(KeyCode::ArrowUp), Some(GbButton::Up));
    assert_eq!(
        keycode_to_gb_button(KeyCode::ArrowDown),
        Some(GbButton::Down)
    );
    assert_eq!(
        keycode_to_gb_button(KeyCode::ArrowLeft),
        Some(GbButton::Left)
    );
    assert_eq!(
        keycode_to_gb_button(KeyCode::ArrowRight),
        Some(GbButton::Right)
    );
    assert_eq!(keycode_to_gb_button(KeyCode::KeyZ), Some(GbButton::A));
    assert_eq!(keycode_to_gb_button(KeyCode::KeyX), Some(GbButton::B));
    assert_eq!(keycode_to_gb_button(KeyCode::Enter), Some(GbButton::Start));
    assert_eq!(
        keycode_to_gb_button(KeyCode::Backspace),
        Some(GbButton::Select)
    );
    assert_eq!(
        keycode_to_gb_button(KeyCode::ShiftRight),
        Some(GbButton::Select)
    );
    assert_eq!(keycode_to_gb_button(KeyCode::KeyA), None);
}

#[test]
fn input_set_from_keycode() {
    let mut input = InputState::new();
    input.set_from_keycode(KeyCode::ArrowUp, true);
    assert!(input.is_held(GbButton::Up));
    input.set_from_keycode(KeyCode::ArrowUp, false);
    assert!(!input.is_held(GbButton::Up));
    input.set_from_keycode(KeyCode::KeyQ, true);
    assert!(!input.any_held());
}

#[test]
fn input_multiple_buttons() {
    let mut input = InputState::new();
    input.press(GbButton::A);
    input.press(GbButton::Up);
    input.press(GbButton::Right);
    assert!(input.is_held(GbButton::A));
    assert!(input.is_held(GbButton::Up));
    assert!(input.is_held(GbButton::Right));
    assert!(!input.is_held(GbButton::B));
    input.release(GbButton::Up);
    assert!(!input.is_held(GbButton::Up));
    assert!(input.is_held(GbButton::A));
}

#[test]
fn gb_button_bit_masks_unique() {
    let mut seen = 0u8;
    for button in GbButton::ALL {
        let mask = button.bit_mask();
        assert_eq!(seen & mask, 0, "duplicate bit mask for {:?}", button);
        seen |= mask;
    }
    assert_eq!(seen, 0xFF);
}

#[test]
fn gb_color_all() {
    assert_eq!(GbColor::ALL.len(), 4);
    assert_eq!(GbColor::ALL[0], GbColor::White);
    assert_eq!(GbColor::ALL[3], GbColor::Black);
}

#[test]
fn obp0_default_makes_color0_transparent() {
    let pal = Palette::from_bgp_register(DEFAULT_OBP0, &GRAYSCALE_PALETTE);
    assert_eq!(pal.colors[0], Rgba::rgb(0xFF, 0xFF, 0xFF));
    assert_eq!(pal.colors[1], Rgba::rgb(0xFF, 0xFF, 0xFF));
    assert_eq!(pal.colors[2], Rgba::rgb(0xAA, 0xAA, 0xAA));
    assert_eq!(pal.colors[3], Rgba::rgb(0x00, 0x00, 0x00));
}

// =====================================================
// M5.2 — Tile, TileMap, Viewport, WindowLayer tests
// =====================================================

use crate::tile::{decode_2bpp_row, Tile, TileSet, BYTES_PER_TILE, TILE_PIXELS};
use crate::tilemap::{BgLayer, TileMap, BG_MAP_HEIGHT, BG_MAP_SIZE, BG_MAP_WIDTH};
use crate::viewport::{ScrollState, Viewport};
use crate::window_layer::WindowLayer;

// --- Tile tests ---

#[test]
fn tile_blank() {
    let t = Tile::blank();
    for row in 0..TILE_PIXELS {
        for col in 0..TILE_PIXELS {
            assert_eq!(t.get(row, col), 0);
        }
    }
}

#[test]
fn tile_decode_2bpp_all_zeros() {
    let data = [0u8; BYTES_PER_TILE];
    let t = Tile::from_2bpp(&data);
    for row in 0..TILE_PIXELS {
        for col in 0..TILE_PIXELS {
            assert_eq!(t.get(row, col), 0, "Expected 0 at ({row}, {col})");
        }
    }
}

#[test]
fn tile_decode_2bpp_all_ones() {
    // lo=0xFF, hi=0xFF for each row → all pixels = color 3
    let data = [0xFF; BYTES_PER_TILE];
    let t = Tile::from_2bpp(&data);
    for row in 0..TILE_PIXELS {
        for col in 0..TILE_PIXELS {
            assert_eq!(t.get(row, col), 3);
        }
    }
}

#[test]
fn tile_decode_2bpp_mixed() {
    // Row 0: lo=0b10000000, hi=0b00000000 → pixel 0 = color 1, rest = 0
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0b10000000; // lo byte, row 0
    data[1] = 0b00000000; // hi byte, row 0
    let t = Tile::from_2bpp(&data);
    assert_eq!(t.get(0, 0), 1); // lo=1, hi=0 → color 1
    assert_eq!(t.get(0, 1), 0);
    assert_eq!(t.get(0, 7), 0);
}

#[test]
fn tile_decode_2bpp_hi_bit() {
    // Row 0: lo=0, hi=0b10000000 → pixel 0 = color 2
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0b00000000;
    data[1] = 0b10000000;
    let t = Tile::from_2bpp(&data);
    assert_eq!(t.get(0, 0), 2); // lo=0, hi=1 → color 2
}

#[test]
fn tile_decode_2bpp_both_bits() {
    // Row 0: lo=0b10000000, hi=0b10000000 → pixel 0 = color 3
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0b10000000;
    data[1] = 0b10000000;
    let t = Tile::from_2bpp(&data);
    assert_eq!(t.get(0, 0), 3);
}

#[test]
fn tile_decode_alternating_pattern() {
    // lo=0b10101010, hi=0b01010101 for row 0
    // bit7: lo=1,hi=0 → 1 | bit6: lo=0,hi=1 → 2 | bit5: lo=1,hi=0 → 1 | ...
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0b10101010;
    data[1] = 0b01010101;
    let t = Tile::from_2bpp(&data);
    assert_eq!(t.get(0, 0), 1); // bit7: lo=1,hi=0
    assert_eq!(t.get(0, 1), 2); // bit6: lo=0,hi=1
    assert_eq!(t.get(0, 2), 1);
    assert_eq!(t.get(0, 3), 2);
    assert_eq!(t.get(0, 4), 1);
    assert_eq!(t.get(0, 5), 2);
    assert_eq!(t.get(0, 6), 1);
    assert_eq!(t.get(0, 7), 2);
}

#[test]
fn tile_flip_x() {
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0b10000000; // only pixel 0 has lo=1
    let t = Tile::from_2bpp(&data);
    let flipped = t.flip_x();
    assert_eq!(flipped.get(0, 7), 1); // pixel 0 → pixel 7
    assert_eq!(flipped.get(0, 0), 0);
}

#[test]
fn tile_flip_y() {
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0xFF; // row 0: all lo=1
    data[1] = 0x00;
    let t = Tile::from_2bpp(&data);
    assert_eq!(t.get(0, 0), 1);
    assert_eq!(t.get(7, 0), 0);
    let flipped = t.flip_y();
    assert_eq!(flipped.get(7, 0), 1); // row 0 → row 7
    assert_eq!(flipped.get(0, 0), 0);
}

#[test]
fn tile_render_row() {
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0xFF; // lo=all 1s for row 0
    data[1] = 0x00; // hi=all 0s → all pixels = color 1
    let t = Tile::from_2bpp(&data);
    let row = t.render_row(0, &GRAYSCALE_PALETTE);
    for pixel in &row {
        assert_eq!(*pixel, Rgba::rgb(0xAA, 0xAA, 0xAA)); // LightGray
    }
}

#[test]
fn decode_2bpp_row_function() {
    let result = decode_2bpp_row(0b11001100, 0b10101010);
    // bit7: lo=1,hi=1 → 3
    // bit6: lo=1,hi=0 → 1
    // bit5: lo=0,hi=1 → 2
    // bit4: lo=0,hi=0 → 0
    // bit3: lo=1,hi=1 → 3
    // bit2: lo=1,hi=0 → 1
    // bit1: lo=0,hi=1 → 2
    // bit0: lo=0,hi=0 → 0
    assert_eq!(result, [3, 1, 2, 0, 3, 1, 2, 0]);
}

// --- TileSet tests ---

#[test]
fn tileset_from_2bpp() {
    // 2 tiles worth of data
    let data = [0u8; BYTES_PER_TILE * 2];
    let ts = TileSet::from_2bpp(&data);
    assert_eq!(ts.len(), 2);
    assert!(!ts.is_empty());
}

#[test]
fn tileset_blank() {
    let ts = TileSet::blank(10);
    assert_eq!(ts.len(), 10);
    for i in 0..10 {
        assert_eq!(ts.get(i).get(0, 0), 0);
    }
}

#[test]
fn tileset_out_of_bounds_returns_blank() {
    let ts = TileSet::blank(1);
    let t = ts.get(999);
    assert_eq!(t.get(0, 0), 0);
}

#[test]
fn tileset_from_1bpp() {
    // 1bpp: 8 bytes per tile, bit=1 → color 3, bit=0 → color 0
    let mut data = [0u8; 8];
    data[0] = 0b10000001; // row 0: pixels 0 and 7 are black
    let ts = TileSet::from_1bpp(&data);
    assert_eq!(ts.len(), 1);
    assert_eq!(ts.get(0).get(0, 0), 3);
    assert_eq!(ts.get(0).get(0, 1), 0);
    assert_eq!(ts.get(0).get(0, 7), 3);
}

#[test]
fn tileset_load_2bpp_at() {
    let mut ts = TileSet::blank(4);
    // Create a tile with all color 3
    let tile_data = [0xFF; BYTES_PER_TILE];
    ts.load_2bpp_at(2, &tile_data);
    assert_eq!(ts.get(0).get(0, 0), 0); // tile 0 still blank
    assert_eq!(ts.get(2).get(0, 0), 3); // tile 2 loaded
}

// --- TileMap tests ---

#[test]
fn tilemap_new_all_zero() {
    let tm = TileMap::new();
    for y in 0..BG_MAP_HEIGHT {
        for x in 0..BG_MAP_WIDTH {
            assert_eq!(tm.get(x, y), 0);
        }
    }
}

#[test]
fn tilemap_set_get() {
    let mut tm = TileMap::new();
    tm.set(5, 10, 42);
    assert_eq!(tm.get(5, 10), 42);
    assert_eq!(tm.get(6, 10), 0);
}

#[test]
fn tilemap_wraps() {
    let mut tm = TileMap::new();
    tm.set(0, 0, 99);
    assert_eq!(tm.get(32, 32), 99); // wraps at 32
    assert_eq!(tm.get(64, 64), 99);
}

#[test]
fn tilemap_from_data() {
    let mut data = vec![0u8; BG_MAP_SIZE];
    data[0] = 5;
    data[31] = 10;
    data[32] = 15; // row 1, col 0
    let tm = TileMap::from_data(&data);
    assert_eq!(tm.get(0, 0), 5);
    assert_eq!(tm.get(31, 0), 10);
    assert_eq!(tm.get(0, 1), 15);
}

#[test]
fn tilemap_fill_rect() {
    let mut tm = TileMap::new();
    tm.fill_rect(2, 3, 4, 5, 77);
    assert_eq!(tm.get(2, 3), 77);
    assert_eq!(tm.get(5, 7), 77);
    assert_eq!(tm.get(1, 3), 0);
    assert_eq!(tm.get(6, 3), 0);
    assert_eq!(tm.get(2, 2), 0);
    assert_eq!(tm.get(2, 8), 0);
}

#[test]
fn tilemap_copy_block() {
    let mut tm = TileMap::new();
    let block = [1, 2, 3, 4, 5, 6]; // 3×2 block
    tm.copy_block(10, 5, &block, 3);
    assert_eq!(tm.get(10, 5), 1);
    assert_eq!(tm.get(11, 5), 2);
    assert_eq!(tm.get(12, 5), 3);
    assert_eq!(tm.get(10, 6), 4);
    assert_eq!(tm.get(11, 6), 5);
    assert_eq!(tm.get(12, 6), 6);
}

#[test]
fn tilemap_render_solid_tile() {
    // Create a tileset with tile 0 = all color 3 (black)
    let tile_data = [0xFF; BYTES_PER_TILE];
    let ts = TileSet::from_2bpp(&tile_data);
    let tm = TileMap::new(); // all tile 0
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    tm.render(&mut fb, &ts, &GRAYSCALE_PALETTE, 0, 0);
    // All pixels should be black (color 3 in grayscale = 0,0,0)
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(159, 143), Some(Rgba::BLACK));
}

#[test]
fn tilemap_render_with_scroll() {
    // Tile 0 = all white (blank), Tile 1 = all black
    let mut data = vec![0u8; BYTES_PER_TILE * 2];
    for i in BYTES_PER_TILE..BYTES_PER_TILE * 2 {
        data[i] = 0xFF;
    }
    let ts = TileSet::from_2bpp(&data);

    let mut tm = TileMap::new();
    // Set tile at (1, 0) to tile 1 (black)
    tm.set(1, 0, 1);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    // Scroll so that tile (1,0) appears at screen (0,0)
    tm.render(&mut fb, &ts, &GRAYSCALE_PALETTE, 8, 0);
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::BLACK));
    // Pixel at screen (8,0) should be tile (2,0) = tile 0 = white
    assert_eq!(fb.get_pixel(8, 0), Some(Rgba::rgb(0xFF, 0xFF, 0xFF)));
}

// --- ScrollState tests ---

#[test]
fn scroll_state_default() {
    let s = ScrollState::default();
    assert_eq!(s.scx, 0);
    assert_eq!(s.scy, 0);
}

#[test]
fn scroll_state_wraps() {
    let s = ScrollState::new(300, 500);
    assert!(s.scx < 256);
    assert!(s.scy < 256);
}

#[test]
fn scroll_state_scroll_by() {
    let mut s = ScrollState::new(0, 0);
    s.scroll_by(10, 20);
    assert_eq!(s.scx, 10);
    assert_eq!(s.scy, 20);
    // Negative scroll wraps
    s.scroll_by(-15, -25);
    assert_eq!(s.scx, 256 - 5); // 251
    assert_eq!(s.scy, 256 - 5); // 251
}

#[test]
fn scroll_state_screen_to_bg() {
    let s = ScrollState::new(100, 50);
    let (bx, by) = s.screen_to_bg(10, 20);
    assert_eq!(bx, 110);
    assert_eq!(by, 70);
}

#[test]
fn scroll_state_screen_to_bg_wraps() {
    let s = ScrollState::new(250, 250);
    let (bx, by) = s.screen_to_bg(10, 10);
    assert_eq!(bx, (250 + 10) % 256);
    assert_eq!(by, (250 + 10) % 256);
}

#[test]
fn scroll_state_bg_to_screen() {
    let s = ScrollState::new(10, 20);
    assert_eq!(s.bg_to_screen(15, 25), Some((5, 5)));
    // Off-screen
    assert_eq!(s.bg_to_screen(200, 200), None);
}

// --- Viewport tests ---

#[test]
fn viewport_from_scroll() {
    let s = ScrollState::new(10, 20);
    let vp = Viewport::from_scroll(&s);
    assert_eq!(vp.x, 10);
    assert_eq!(vp.y, 20);
    assert_eq!(vp.width, SCREEN_WIDTH);
    assert_eq!(vp.height, SCREEN_HEIGHT);
}

// --- WindowLayer tests ---

#[test]
fn window_layer_disabled_by_default() {
    let wl = WindowLayer::new();
    assert!(!wl.enabled);
    assert_eq!(wl.wx, 7);
    assert_eq!(wl.wy, 0);
}

#[test]
fn window_layer_screen_x() {
    let mut wl = WindowLayer::new();
    wl.wx = 7;
    assert_eq!(wl.screen_x(), 0);
    wl.wx = 87;
    assert_eq!(wl.screen_x(), 80);
    wl.wx = 3; // less than 7, clamp to 0
    assert_eq!(wl.screen_x(), 0);
}

#[test]
fn window_layer_render_disabled() {
    let wl = WindowLayer::new();
    let ts = TileSet::blank(1);
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    wl.render(&mut fb, &ts, &GRAYSCALE_PALETTE);
    // Should not change anything since disabled
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE));
}

#[test]
fn window_layer_render_enabled() {
    let mut wl = WindowLayer::new();
    wl.enabled = true;
    wl.wx = 7; // starts at screen x=0
    wl.wy = 0; // starts at screen y=0

    // Tile 0 = all black
    let tile_data = [0xFF; BYTES_PER_TILE];
    let ts = TileSet::from_2bpp(&tile_data);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    wl.render(&mut fb, &ts, &GRAYSCALE_PALETTE);
    // Window covers entire screen with black tile
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(159, 143), Some(Rgba::BLACK));
}

#[test]
fn window_layer_partial_coverage() {
    let mut wl = WindowLayer::new();
    wl.enabled = true;
    wl.wx = 7 + 80; // starts at screen x=80
    wl.wy = 72; // starts at screen y=72

    let tile_data = [0xFF; BYTES_PER_TILE];
    let ts = TileSet::from_2bpp(&tile_data);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    wl.render(&mut fb, &ts, &GRAYSCALE_PALETTE);

    // Above and left of window should remain white
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE));
    assert_eq!(fb.get_pixel(79, 71), Some(Rgba::WHITE));
    // Window area should be black
    assert_eq!(fb.get_pixel(80, 72), Some(Rgba::BLACK));
    assert_eq!(fb.get_pixel(159, 143), Some(Rgba::BLACK));
}

// --- BgLayer tests ---

#[test]
fn bg_layer_default() {
    let bg = BgLayer::new();
    assert_eq!(bg.scroll_x, 0);
    assert_eq!(bg.scroll_y, 0);
}

// =====================================================
// M5.3 — Sprite rendering tests
// =====================================================

use crate::sprite::{
    SpriteLayer, SpriteOamEntry, OAM_PALETTE, OAM_PRIORITY, OAM_X_FLIP, OAM_Y_FLIP,
};

#[test]
fn sprite_oam_from_raw() {
    let e = SpriteOamEntry::from_raw(32, 24, 0, 0);
    assert_eq!(e.y, 16); // 32 - 16
    assert_eq!(e.x, 16); // 24 - 8
}

#[test]
fn sprite_oam_attributes() {
    let e = SpriteOamEntry::new(
        0,
        0,
        0,
        OAM_PRIORITY | OAM_Y_FLIP | OAM_X_FLIP | OAM_PALETTE,
    );
    assert!(e.bg_priority());
    assert!(e.y_flip());
    assert!(e.x_flip());
    assert!(e.uses_obp1());

    let e2 = SpriteOamEntry::new(0, 0, 0, 0);
    assert!(!e2.bg_priority());
    assert!(!e2.y_flip());
    assert!(!e2.x_flip());
    assert!(!e2.uses_obp1());
}

#[test]
fn sprite_on_screen() {
    let on = SpriteOamEntry::new(0, 0, 0, 0);
    assert!(on.is_on_screen());

    let off_right = SpriteOamEntry::new(0, 160, 0, 0);
    assert!(!off_right.is_on_screen());

    let off_bottom = SpriteOamEntry::new(144, 0, 0, 0);
    assert!(!off_bottom.is_on_screen());

    let partially_on = SpriteOamEntry::new(-4, -4, 0, 0);
    assert!(partially_on.is_on_screen());

    let fully_off_left = SpriteOamEntry::new(0, -8, 0, 0);
    assert!(!fully_off_left.is_on_screen());
}

#[test]
fn sprite_render_transparent_color0() {
    // Tile with all color 0 = transparent → nothing drawn
    let ts = TileSet::blank(1);
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, 0));
    layer.render(&mut fb, &ts, &GRAYSCALE_PALETTE, &GRAYSCALE_PALETTE, None);
    // Should still be white (color 0 is transparent)
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE));
}

#[test]
fn sprite_render_visible() {
    // Tile 0 = all color 3 (black)
    let tile_data = [0xFF; BYTES_PER_TILE];
    let ts = TileSet::from_2bpp(&tile_data);
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(10, 20, 0, 0));
    layer.render(&mut fb, &ts, &GRAYSCALE_PALETTE, &GRAYSCALE_PALETTE, None);
    // Pixel (20, 10) should be black
    assert_eq!(fb.get_pixel(20, 10), Some(Rgba::BLACK));
    // Pixel outside sprite should be white
    assert_eq!(fb.get_pixel(19, 10), Some(Rgba::WHITE));
    assert_eq!(fb.get_pixel(28, 10), Some(Rgba::WHITE));
}

#[test]
fn sprite_render_x_flip() {
    // Create a tile where only column 0 has color 1
    let mut data = [0u8; BYTES_PER_TILE];
    for row in 0..8 {
        data[row * 2] = 0b10000000; // lo: only bit 7 (col 0) set
        data[row * 2 + 1] = 0;
    }
    let ts = TileSet::from_2bpp(&data);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, OAM_X_FLIP));
    layer.render(&mut fb, &ts, &GRAYSCALE_PALETTE, &GRAYSCALE_PALETTE, None);

    // With x-flip, column 0 becomes column 7
    assert_eq!(fb.get_pixel(7, 0), Some(Rgba::rgb(0xAA, 0xAA, 0xAA))); // color 1 = light gray
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE)); // transparent (color 0)
}

#[test]
fn sprite_render_y_flip() {
    // Create a tile where only row 0 has color 3
    let mut data = [0u8; BYTES_PER_TILE];
    data[0] = 0xFF; // row 0 lo
    data[1] = 0xFF; // row 0 hi → color 3
    let ts = TileSet::from_2bpp(&data);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, OAM_Y_FLIP));
    layer.render(&mut fb, &ts, &GRAYSCALE_PALETTE, &GRAYSCALE_PALETTE, None);

    // With y-flip, row 0 → row 7
    assert_eq!(fb.get_pixel(0, 7), Some(Rgba::BLACK)); // color 3
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE)); // transparent (was row 7, now row 0)
}

#[test]
fn sprite_render_obp1() {
    // Use OBP1 palette
    let tile_data = [0xFF; BYTES_PER_TILE]; // all color 3
    let ts = TileSet::from_2bpp(&tile_data);

    let obp0 = Palette::new([Rgba::WHITE, Rgba::WHITE, Rgba::WHITE, Rgba::rgb(0xAA, 0, 0)]);
    let obp1 = Palette::new([Rgba::WHITE, Rgba::WHITE, Rgba::WHITE, Rgba::rgb(0, 0, 0xBB)]);

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, OAM_PALETTE));
    layer.render(&mut fb, &ts, &obp0, &obp1, None);

    // Should use OBP1's color 3 (blue)
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::rgb(0, 0, 0xBB)));
}

#[test]
fn sprite_bg_priority() {
    let tile_data = [0xFF; BYTES_PER_TILE]; // all color 3
    let ts = TileSet::from_2bpp(&tile_data);

    // BG color buffer: pixel at (0,0) has BG color 2 (non-zero)
    let mut bg_buf = vec![0u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    bg_buf[0] = 2; // pixel (0,0) has non-zero BG

    let mut fb = FrameBuffer::new(Rgba::WHITE);
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, OAM_PRIORITY));
    layer.render(
        &mut fb,
        &ts,
        &GRAYSCALE_PALETTE,
        &GRAYSCALE_PALETTE,
        Some(&bg_buf),
    );

    // Pixel (0,0) should NOT be overwritten (BG priority, BG != 0)
    assert_eq!(fb.get_pixel(0, 0), Some(Rgba::WHITE));
    // Pixel (1,0) has BG color 0, so sprite draws there
    assert_eq!(fb.get_pixel(1, 0), Some(Rgba::BLACK));
}

#[test]
fn sprite_layer_clear() {
    let mut layer = SpriteLayer::new();
    layer.add(SpriteOamEntry::new(0, 0, 0, 0));
    layer.add(SpriteOamEntry::new(10, 10, 0, 0));
    assert_eq!(layer.entries.len(), 2);
    layer.clear();
    assert_eq!(layer.entries.len(), 0);
}
