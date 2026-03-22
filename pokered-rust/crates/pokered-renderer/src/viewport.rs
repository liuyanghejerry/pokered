//! Viewport / scroll state — controls which part of the 256×256 BG map is visible.
//!
//! The Game Boy's SCX/SCY registers scroll the background.
//! This module tracks scroll position and provides coordinate conversion.

use crate::tilemap::BG_MAP_PIXEL_HEIGHT;
use crate::tilemap::BG_MAP_PIXEL_WIDTH;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// Scroll state for the background layer.
///
/// On the real Game Boy, SCX and SCY are 8-bit registers (0–255),
/// and the background wraps at 256 pixels. We use u32 for convenience
/// but always wrap to 0–255.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollState {
    /// Horizontal scroll (SCX equivalent), 0–255
    pub scx: u32,
    /// Vertical scroll (SCY equivalent), 0–255
    pub scy: u32,
}

impl ScrollState {
    pub fn new(scx: u32, scy: u32) -> Self {
        Self {
            scx: scx % BG_MAP_PIXEL_WIDTH,
            scy: scy % BG_MAP_PIXEL_HEIGHT,
        }
    }

    /// Set scroll position, wrapping to valid range.
    pub fn set(&mut self, scx: u32, scy: u32) {
        self.scx = scx % BG_MAP_PIXEL_WIDTH;
        self.scy = scy % BG_MAP_PIXEL_HEIGHT;
    }

    /// Scroll by a delta, wrapping around.
    pub fn scroll_by(&mut self, dx: i32, dy: i32) {
        self.scx = ((self.scx as i32 + dx).rem_euclid(BG_MAP_PIXEL_WIDTH as i32)) as u32;
        self.scy = ((self.scy as i32 + dy).rem_euclid(BG_MAP_PIXEL_HEIGHT as i32)) as u32;
    }

    /// Convert screen coordinate to background coordinate.
    #[inline]
    pub fn screen_to_bg(&self, screen_x: u32, screen_y: u32) -> (u32, u32) {
        let bg_x = (self.scx + screen_x) % BG_MAP_PIXEL_WIDTH;
        let bg_y = (self.scy + screen_y) % BG_MAP_PIXEL_HEIGHT;
        (bg_x, bg_y)
    }

    /// Convert background coordinate to screen coordinate.
    /// Returns None if the bg position is not visible on screen.
    pub fn bg_to_screen(&self, bg_x: u32, bg_y: u32) -> Option<(u32, u32)> {
        // Calculate the relative position from scroll origin
        let rel_x = ((bg_x as i32 - self.scx as i32).rem_euclid(BG_MAP_PIXEL_WIDTH as i32)) as u32;
        let rel_y = ((bg_y as i32 - self.scy as i32).rem_euclid(BG_MAP_PIXEL_HEIGHT as i32)) as u32;

        if rel_x < SCREEN_WIDTH && rel_y < SCREEN_HEIGHT {
            Some((rel_x, rel_y))
        } else {
            None
        }
    }
}

impl Default for ScrollState {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Viewport rectangle representing the visible area in BG coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    /// Create a viewport from scroll state.
    pub fn from_scroll(scroll: &ScrollState) -> Self {
        Self {
            x: scroll.scx,
            y: scroll.scy,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    /// Check if a bg-space rectangle intersects this viewport (accounting for wrapping).
    pub fn intersects_bg_rect(&self, rect_x: u32, rect_y: u32, rect_w: u32, rect_h: u32) -> bool {
        // Simple check: for each screen pixel, see if it falls in the rect
        // This is a conservative check — for wrapping maps, full intersection
        // testing is complex. We do a quick bounding check in unwrapped space.
        let vp_right = self.x + self.width;
        let vp_bottom = self.y + self.height;
        let rect_right = rect_x + rect_w;
        let rect_bottom = rect_y + rect_h;

        // Non-wrapping case (most common in Pokémon Red)
        if vp_right <= BG_MAP_PIXEL_WIDTH && rect_right <= BG_MAP_PIXEL_WIDTH {
            return !(rect_right <= self.x
                || rect_x >= vp_right
                || rect_bottom <= self.y
                || rect_y >= vp_bottom);
        }

        // Wrapping case: conservatively return true
        true
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }
}
