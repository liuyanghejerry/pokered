//! Lightweight layout engine for Game Boy screen coordinates.
//!
//! Game Boy has several coordinate systems that need conversion:
//! - **Screen coordinates**: Pixel positions (0-159 X, 0-143 Y)
//! - **Tile coordinates**: Tile positions (0-19 X, 0-17 Y), each tile is 8x8 pixels
//! - **OAM coordinates**: Sprite position registers with built-in offsets
//!
//! This module provides utilities to convert between these systems and define
//! common layout positions.

/// Screen dimensions in pixels.
pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

/// Tile dimensions.
pub const TILE_SIZE: u32 = 8;

/// Screen dimensions in tiles.
pub const SCREEN_WIDTH_TILES: u32 = SCREEN_WIDTH / TILE_SIZE; // 20
pub const SCREEN_HEIGHT_TILES: u32 = SCREEN_HEIGHT / TILE_SIZE; // 18

/// OAM (Object Attribute Memory) offset constants.
/// Game Boy sprites use these offsets to allow sprites to be partially off-screen.
pub const OAM_Y_OFFSET: u32 = 16;
pub const OAM_X_OFFSET: u32 = 8;

// ============================================================================
// Coordinate conversions
// ============================================================================

/// Convert tile coordinates to pixel coordinates.
/// `tile_x` and `tile_y` are in tile units (0-19, 0-17).
/// Returns pixel coordinates of the tile's top-left corner.
#[inline]
pub fn tile_to_pixel(tile_x: u32, tile_y: u32) -> (u32, u32) {
    (tile_x * TILE_SIZE, tile_y * TILE_SIZE)
}

/// Convert pixel coordinates to tile coordinates.
/// Returns the tile that contains the given pixel.
#[inline]
pub fn pixel_to_tile(pixel_x: u32, pixel_y: u32) -> (u32, u32) {
    (pixel_x / TILE_SIZE, pixel_y / TILE_SIZE)
}

/// Convert OAM coordinates to screen coordinates.
/// OAM Y = screen_y + 16, OAM X = screen_x + 8.
/// This allows sprites to be positioned partially off the top/left edges.
#[inline]
pub fn oam_to_screen(oam_y: u32, oam_x: u32) -> (u32, u32) {
    let screen_y = oam_y.saturating_sub(OAM_Y_OFFSET);
    let screen_x = oam_x.saturating_sub(OAM_X_OFFSET);
    (screen_x, screen_y)
}

/// Convert screen coordinates to OAM coordinates.
/// OAM Y = screen_y + 16, OAM X = screen_x + 8.
#[inline]
pub fn screen_to_oam(screen_y: u32, screen_x: u32) -> (u32, u32) {
    (screen_y + OAM_Y_OFFSET, screen_x + OAM_X_OFFSET)
}

/// Convert OAM coordinates (as i16 for signed math) to screen coordinates.
/// Returns None if the sprite is completely off-screen.
#[inline]
pub fn oam_to_screen_signed(oam_y: i16, oam_x: i16) -> Option<(i32, i32)> {
    let screen_y = oam_y as i32 - OAM_Y_OFFSET as i32;
    let screen_x = oam_x as i32 - OAM_X_OFFSET as i32;

    // Check if sprite is visible (within screen bounds)
    if screen_y < -16 || screen_y >= SCREEN_HEIGHT as i32 {
        return None;
    }
    if screen_x < -8 || screen_x >= SCREEN_WIDTH as i32 {
        return None;
    }

    Some((screen_x, screen_y))
}

// ============================================================================
// Common title screen positions
// ============================================================================

/// Title screen layout constants.
/// Based on `engine/movie/title.asm` positions.
pub mod title_screen {
    use super::*;

    /// Pokemon logo position (from hlcoord 2, 1).
    /// Logo is 16 tiles wide × 7 tiles tall.
    pub const LOGO_TILE_X: u32 = 2;
    pub const LOGO_TILE_Y: u32 = 1;
    pub const LOGO_PIXEL_X: u32 = LOGO_TILE_X * TILE_SIZE; // 16
    pub const LOGO_PIXEL_Y: u32 = LOGO_TILE_Y * TILE_SIZE; // 8

    /// Version text position (from hlcoord 7, 8).
    /// "Red Version" or "Blue Version".
    pub const VERSION_TILE_X: u32 = 7;
    pub const VERSION_TILE_Y: u32 = 8;
    pub const VERSION_PIXEL_X: u32 = VERSION_TILE_X * TILE_SIZE; // 56
    pub const VERSION_PIXEL_Y: u32 = VERSION_TILE_Y * TILE_SIZE; // 64

    /// Pokemon sprite position (from hlcoord 5, 10).
    /// This is where the front sprite is drawn on the title screen.
    pub const POKEMON_TILE_X: u32 = 5;
    pub const POKEMON_TILE_Y: u32 = 10;
    pub const POKEMON_PIXEL_X: u32 = POKEMON_TILE_X * TILE_SIZE; // 40
    pub const POKEMON_PIXEL_Y: u32 = POKEMON_TILE_Y * TILE_SIZE; // 80

    /// Player character sprite position (from OAM Y=$60, X=$5a).
    /// Player sprite is 5 tiles wide × 7 tiles tall (40×56 pixels).
    pub const PLAYER_OAM_Y: u32 = 0x60; // 96
    pub const PLAYER_OAM_X: u32 = 0x5a; // 90

    /// Get player sprite screen position from OAM values.
    pub fn player_screen_pos() -> (u32, u32) {
        super::oam_to_screen(PLAYER_OAM_Y, PLAYER_OAM_X)
    }

    /// Copyright text position (from hlcoord 2, 17).
    pub const COPYRIGHT_TILE_X: u32 = 2;
    pub const COPYRIGHT_TILE_Y: u32 = 17;
    pub const COPYRIGHT_PIXEL_X: u32 = COPYRIGHT_TILE_X * TILE_SIZE; // 16
    pub const COPYRIGHT_PIXEL_Y: u32 = COPYRIGHT_TILE_Y * TILE_SIZE; // 136
}

/// Copyright splash screen positions.
/// From `engine/movie/title.asm` LoadCopyrightTiles.
pub mod copyright_screen {
    use super::*;

    /// Copyright text position (from hlcoord 2, 7).
    pub const TEXT_TILE_X: u32 = 2;
    pub const TEXT_TILE_Y: u32 = 7;
    pub const TEXT_PIXEL_X: u32 = TEXT_TILE_X * TILE_SIZE; // 16
    pub const TEXT_PIXEL_Y: u32 = TEXT_TILE_Y * TILE_SIZE; // 56
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_to_pixel() {
        assert_eq!(tile_to_pixel(0, 0), (0, 0));
        assert_eq!(tile_to_pixel(2, 1), (16, 8));
        assert_eq!(tile_to_pixel(7, 8), (56, 64));
        assert_eq!(tile_to_pixel(5, 10), (40, 80));
    }

    #[test]
    fn test_oam_to_screen() {
        // Player sprite from title.asm: OAM Y=$60, X=$5a
        assert_eq!(oam_to_screen(0x60, 0x5a), (82, 80));

        // Test edge cases
        assert_eq!(oam_to_screen(16, 8), (0, 0));
        assert_eq!(oam_to_screen(0, 0), (0, 0)); // saturating_sub
    }

    #[test]
    fn test_screen_to_oam() {
        assert_eq!(screen_to_oam(80, 82), (96, 90)); // (Y, X) -> (OAM_Y, OAM_X)
    }

    #[test]
    fn test_title_screen_positions() {
        // Logo at tile (2, 1) = pixel (16, 8)
        assert_eq!(title_screen::LOGO_PIXEL_X, 16);
        assert_eq!(title_screen::LOGO_PIXEL_Y, 8);

        // Version text at tile (7, 8) = pixel (56, 64)
        assert_eq!(title_screen::VERSION_PIXEL_X, 56);
        assert_eq!(title_screen::VERSION_PIXEL_Y, 64);

        // Pokemon at tile (5, 10) = pixel (40, 80)
        assert_eq!(title_screen::POKEMON_PIXEL_X, 40);
        assert_eq!(title_screen::POKEMON_PIXEL_Y, 80);

        // Player from OAM
        assert_eq!(title_screen::player_screen_pos(), (82, 80));
    }
}
