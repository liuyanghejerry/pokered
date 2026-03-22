pub mod processor;
pub mod renderer;

#[cfg(test)]
mod tests;

use pokered_data::charmap;

pub const SCREEN_WIDTH: u8 = 20;
pub const SCREEN_HEIGHT: u8 = 18;
pub const TEXT_BOX_WIDTH: u8 = 18;
pub const TEXT_BOX_LINES: u8 = 2;

// ── Tilemap Position ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileCoord {
    pub x: u8,
    pub y: u8,
}

impl TileCoord {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn to_tilemap_index(self) -> usize {
        self.y as usize * SCREEN_WIDTH as usize + self.x as usize
    }

    pub fn from_tilemap_index(index: usize) -> Self {
        Self {
            x: (index % SCREEN_WIDTH as usize) as u8,
            y: (index / SCREEN_WIDTH as usize) as u8,
        }
    }
}

// ── Text Box ──────────────────────────────────────────────────────

pub const TILE_TOP_LEFT: u8 = 0x79; // ┌
pub const TILE_TOP_RIGHT: u8 = 0x7B; // ┐
pub const TILE_BOTTOM_LEFT: u8 = 0x7C; // └
pub const TILE_BOTTOM_RIGHT: u8 = 0x7E; // ┘
pub const TILE_HORIZONTAL: u8 = 0x7A; // ─
pub const TILE_VERTICAL: u8 = 0x7F; // │
pub const TILE_SPACE: u8 = 0x7F;
pub const TILE_DOWN_ARROW: u8 = 0xED; // ▼

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBox {
    pub origin: TileCoord,
    pub width: u8,
    pub height: u8,
}

impl TextBox {
    pub fn new(origin: TileCoord, width: u8, height: u8) -> Self {
        Self {
            origin,
            width,
            height,
        }
    }

    pub fn standard_dialog() -> Self {
        Self {
            origin: TileCoord::new(0, 12),
            width: SCREEN_WIDTH,
            height: 6,
        }
    }

    pub fn text_start_coord(&self) -> TileCoord {
        TileCoord::new(self.origin.x + 1, self.origin.y + 2)
    }

    pub fn second_line_coord(&self) -> TileCoord {
        TileCoord::new(self.origin.x + 1, self.origin.y + 4)
    }

    pub fn arrow_coord(&self) -> TileCoord {
        TileCoord::new(
            self.origin.x + self.width - 2,
            self.origin.y + self.height - 2,
        )
    }
}

// ── Tilemap Buffer ────────────────────────────────────────────────

pub const TILEMAP_SIZE: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;

#[derive(Clone)]
pub struct TilemapBuffer {
    pub tiles: [u8; TILEMAP_SIZE],
}

impl Default for TilemapBuffer {
    fn default() -> Self {
        Self {
            tiles: [charmap::CHAR_SPACE; TILEMAP_SIZE],
        }
    }
}

impl TilemapBuffer {
    pub fn get(&self, coord: TileCoord) -> u8 {
        self.tiles[coord.to_tilemap_index()]
    }

    pub fn set(&mut self, coord: TileCoord, tile: u8) {
        self.tiles[coord.to_tilemap_index()] = tile;
    }

    pub fn draw_box_border(&mut self, text_box: &TextBox) {
        let x = text_box.origin.x;
        let y = text_box.origin.y;
        let w = text_box.width;
        let h = text_box.height;
        let inner_w = w - 2;

        // Top row
        self.set(TileCoord::new(x, y), TILE_TOP_LEFT);
        for i in 0..inner_w {
            self.set(TileCoord::new(x + 1 + i, y), TILE_HORIZONTAL);
        }
        self.set(TileCoord::new(x + w - 1, y), TILE_TOP_RIGHT);

        // Middle rows
        for row in 1..h - 1 {
            self.set(TileCoord::new(x, y + row), TILE_VERTICAL);
            for col in 0..inner_w {
                self.set(TileCoord::new(x + 1 + col, y + row), charmap::CHAR_SPACE);
            }
            self.set(TileCoord::new(x + w - 1, y + row), TILE_VERTICAL);
        }

        // Bottom row
        self.set(TileCoord::new(x, y + h - 1), TILE_BOTTOM_LEFT);
        for i in 0..inner_w {
            self.set(TileCoord::new(x + 1 + i, y + h - 1), TILE_HORIZONTAL);
        }
        self.set(TileCoord::new(x + w - 1, y + h - 1), TILE_BOTTOM_RIGHT);
    }

    pub fn clear_area(&mut self, origin: TileCoord, width: u8, height: u8) {
        for row in 0..height {
            for col in 0..width {
                self.set(
                    TileCoord::new(origin.x + col, origin.y + row),
                    charmap::CHAR_SPACE,
                );
            }
        }
    }

    pub fn scroll_lines_up(&mut self, src_y: u8, num_rows: u8) {
        for row in 0..num_rows {
            let src = (src_y + row) as usize * SCREEN_WIDTH as usize;
            let dst = (src_y + row - 1) as usize * SCREEN_WIDTH as usize;
            for col in 0..SCREEN_WIDTH as usize {
                self.tiles[dst + col] = self.tiles[src + col];
            }
        }
        // Clear last source row
        let last = (src_y + num_rows - 1) as usize * SCREEN_WIDTH as usize;
        for col in 1..(SCREEN_WIDTH - 1) as usize {
            self.tiles[last + col] = charmap::CHAR_SPACE;
        }
    }
}

// ── Text Engine State ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextState {
    Idle,
    Printing,
    WaitButton,
    WaitScroll,
    Paused { frames_remaining: u8 },
    AsmCallback,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextResult {
    Continue,
    WaitForButton,
    WaitForScroll,
    Pause(u8),
    StartAsm,
    Done,
}

pub struct TextEngine {
    pub state: TextState,
    pub cursor: TileCoord,
    pub text_box: TextBox,
    pub letter_delay_enabled: bool,
    pub letter_delay_frames: u8,
    pub frames_since_last_char: u8,
    pub single_spaced_lines: bool,
}

impl TextEngine {
    pub fn new() -> Self {
        let text_box = TextBox::standard_dialog();
        Self {
            state: TextState::Idle,
            cursor: text_box.text_start_coord(),
            text_box,
            letter_delay_enabled: true,
            letter_delay_frames: 1,
            frames_since_last_char: 0,
            single_spaced_lines: false,
        }
    }

    pub fn start(&mut self, text_box: TextBox) {
        self.text_box = text_box;
        self.cursor = text_box.text_start_coord();
        self.state = TextState::Printing;
        self.frames_since_last_char = 0;
    }

    pub fn is_idle(&self) -> bool {
        self.state == TextState::Idle
    }

    pub fn is_done(&self) -> bool {
        self.state == TextState::Done
    }

    pub fn advance_cursor(&mut self) {
        self.cursor.x += 1;
    }

    pub fn move_to_next_line(&mut self) {
        let line_spacing = if self.single_spaced_lines { 1 } else { 2 };
        self.cursor.x = self.text_box.origin.x + 1;
        self.cursor.y += line_spacing;
    }

    pub fn move_to_second_line(&mut self) {
        self.cursor = self.text_box.second_line_coord();
    }

    pub fn move_to_coord(&mut self, coord: TileCoord) {
        self.cursor = coord;
    }

    pub fn confirm_button_press(&mut self) {
        match self.state {
            TextState::WaitButton => {
                self.state = TextState::Printing;
            }
            TextState::WaitScroll => {
                self.state = TextState::Printing;
            }
            _ => {}
        }
    }

    pub fn tick_pause(&mut self) -> bool {
        if let TextState::Paused {
            ref mut frames_remaining,
        } = self.state
        {
            if *frames_remaining > 0 {
                *frames_remaining -= 1;
                if *frames_remaining == 0 {
                    self.state = TextState::Printing;
                    return true;
                }
            }
        }
        false
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── Name Buffer ───────────────────────────────────────────────────

pub const NAME_LENGTH: usize = 11;

pub struct NameBuffers {
    pub player_name: [u8; NAME_LENGTH],
    pub rival_name: [u8; NAME_LENGTH],
}

impl Default for NameBuffers {
    fn default() -> Self {
        let mut player = [charmap::CHAR_TERMINATOR; NAME_LENGTH];
        let mut rival = [charmap::CHAR_TERMINATOR; NAME_LENGTH];
        // Default names
        let p = charmap::encode_string("RED").unwrap_or_default();
        let r = charmap::encode_string("BLUE").unwrap_or_default();
        for (i, &b) in p.iter().enumerate().take(NAME_LENGTH) {
            player[i] = b;
        }
        for (i, &b) in r.iter().enumerate().take(NAME_LENGTH) {
            rival[i] = b;
        }
        Self {
            player_name: player,
            rival_name: rival,
        }
    }
}
