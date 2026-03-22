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
