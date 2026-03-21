use pokered_data::charmap;
use pokered_data::text_commands::{inline_control_chars, TextCommand, TX_END};

use super::{
    NameBuffers, TextEngine, TextResult, TextState, TileCoord, TilemapBuffer, TILE_DOWN_ARROW,
};

pub struct TextProcessor<'a> {
    pub engine: &'a mut TextEngine,
    pub tilemap: &'a mut TilemapBuffer,
    pub names: &'a NameBuffers,
}

#[derive(Debug, Clone)]
pub struct TextStream {
    data: Vec<u8>,
    pos: usize,
}

impl TextStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn peek(&self) -> Option<u8> {
        self.data.get(self.pos).copied()
    }

    pub fn read(&mut self) -> Option<u8> {
        let byte = self.data.get(self.pos).copied();
        if byte.is_some() {
            self.pos += 1;
        }
        byte
    }

    pub fn read_u16_le(&mut self) -> Option<u16> {
        let lo = self.read()? as u16;
        let hi = self.read()? as u16;
        Some((hi << 8) | lo)
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn remaining(&self) -> &[u8] {
        &self.data[self.pos..]
    }
}

impl<'a> TextProcessor<'a> {
    pub fn new(
        engine: &'a mut TextEngine,
        tilemap: &'a mut TilemapBuffer,
        names: &'a NameBuffers,
    ) -> Self {
        Self {
            engine,
            tilemap,
            names,
        }
    }

    pub fn process_next_byte(&mut self, stream: &mut TextStream) -> TextResult {
        let byte = match stream.read() {
            Some(b) => b,
            None => {
                self.engine.state = TextState::Done;
                return TextResult::Done;
            }
        };

        if byte == TX_END || byte == charmap::CHAR_TERMINATOR {
            self.engine.state = TextState::Done;
            return TextResult::Done;
        }

        if let Some(cmd) = TextCommand::from_byte(byte) {
            return self.handle_text_command(cmd, stream);
        }

        if inline_control_chars::is_inline_control(byte) {
            return self.handle_inline_control(byte, stream);
        }

        self.place_char(byte);
        TextResult::Continue
    }

    fn handle_text_command(&mut self, cmd: TextCommand, stream: &mut TextStream) -> TextResult {
        match cmd {
            TextCommand::TxStart => {
                self.process_text_until_terminator(stream);
                TextResult::Continue
            }
            TextCommand::TxRam => {
                // In Rust we skip RAM pointer — read and discard 2 bytes
                let _addr = stream.read_u16_le();
                TextResult::Continue
            }
            TextCommand::TxBcd => {
                let _addr = stream.read_u16_le();
                let _flags = stream.read();
                TextResult::Continue
            }
            TextCommand::TxMove => {
                if let Some(addr) = stream.read_u16_le() {
                    let tilemap_offset = addr.wrapping_sub(0x9800) as usize;
                    let coord =
                        TileCoord::from_tilemap_index(tilemap_offset.min(super::TILEMAP_SIZE - 1));
                    self.engine.move_to_coord(coord);
                }
                TextResult::Continue
            }
            TextCommand::TxBox => {
                if let (Some(addr), Some(height), Some(width)) =
                    (stream.read_u16_le(), stream.read(), stream.read())
                {
                    let tilemap_offset = addr.wrapping_sub(0x9800) as usize;
                    let coord =
                        TileCoord::from_tilemap_index(tilemap_offset.min(super::TILEMAP_SIZE - 1));
                    let text_box = super::TextBox::new(coord, width + 2, height + 2);
                    self.tilemap.draw_box_border(&text_box);
                }
                TextResult::Continue
            }
            TextCommand::TxLow => {
                // coord(1, 16) — bottom line of standard dialog
                self.engine.move_to_coord(TileCoord::new(1, 16));
                TextResult::Continue
            }
            TextCommand::TxPromptButton => {
                let arrow_pos = self.engine.text_box.arrow_coord();
                self.tilemap.set(arrow_pos, TILE_DOWN_ARROW);
                self.engine.state = TextState::WaitButton;
                TextResult::WaitForButton
            }
            TextCommand::TxScroll => {
                self.scroll_text_up();
                self.engine.move_to_coord(TileCoord::new(1, 16));
                TextResult::Continue
            }
            TextCommand::TxStartAsm => {
                self.engine.state = TextState::AsmCallback;
                TextResult::StartAsm
            }
            TextCommand::TxNum => {
                let _addr = stream.read_u16_le();
                let _flags = stream.read();
                TextResult::Continue
            }
            TextCommand::TxPause => {
                self.engine.state = TextState::Paused {
                    frames_remaining: 30,
                };
                TextResult::Pause(30)
            }
            TextCommand::TxDots => {
                let count = stream.read().unwrap_or(0);
                let dot_char = charmap::encode_char('…').unwrap_or(0xE2);
                for _ in 0..count {
                    self.place_char(dot_char);
                }
                TextResult::Continue
            }
            TextCommand::TxWaitButton => {
                self.engine.state = TextState::WaitButton;
                TextResult::WaitForButton
            }
            TextCommand::TxFar => {
                // Read 3-byte far pointer (addr_lo, addr_hi, bank) — skip bank in Rust
                let _addr = stream.read_u16_le();
                let _bank = stream.read();
                TextResult::Continue
            }
            _ if cmd.is_sound_command() => {
                // Sound commands: no-op in text processor (handled by audio system)
                TextResult::Continue
            }
            _ => TextResult::Continue,
        }
    }

    fn handle_inline_control(&mut self, byte: u8, _stream: &mut TextStream) -> TextResult {
        match byte {
            inline_control_chars::NEXT => {
                self.engine.move_to_next_line();
                TextResult::Continue
            }
            inline_control_chars::LINE => {
                self.engine.move_to_coord(TileCoord::new(1, 16));
                TextResult::Continue
            }
            inline_control_chars::PARA => {
                let arrow_pos = self.engine.text_box.arrow_coord();
                self.tilemap.set(arrow_pos, TILE_DOWN_ARROW);
                self.engine.state = TextState::WaitButton;
                TextResult::WaitForButton
            }
            inline_control_chars::PAGE => {
                let arrow_pos = self.engine.text_box.arrow_coord();
                self.tilemap.set(arrow_pos, TILE_DOWN_ARROW);
                self.engine.state = TextState::WaitButton;
                TextResult::WaitForButton
            }
            inline_control_chars::CONT => {
                let arrow_pos = self.engine.text_box.arrow_coord();
                self.tilemap.set(arrow_pos, TILE_DOWN_ARROW);
                self.engine.state = TextState::WaitScroll;
                TextResult::WaitForScroll
            }
            inline_control_chars::DONE => {
                self.engine.state = TextState::Done;
                TextResult::Done
            }
            inline_control_chars::PROMPT => {
                let arrow_pos = self.engine.text_box.arrow_coord();
                self.tilemap.set(arrow_pos, TILE_DOWN_ARROW);
                self.engine.state = TextState::WaitButton;
                TextResult::WaitForButton
            }
            inline_control_chars::PLAYER => {
                self.place_name_string(&self.names.player_name.clone());
                TextResult::Continue
            }
            inline_control_chars::RIVAL => {
                self.place_name_string(&self.names.rival_name.clone());
                TextResult::Continue
            }
            inline_control_chars::TARGET | inline_control_chars::USER => {
                // Battle context names — placeholder, handled by battle system
                TextResult::Continue
            }
            inline_control_chars::POKE => {
                let poke_text = charmap::encode_string("POKé").unwrap_or_default();
                for &b in &poke_text[..poke_text.len().saturating_sub(1)] {
                    self.place_char(b);
                }
                TextResult::Continue
            }
            inline_control_chars::DEXEND => {
                let period = charmap::encode_char('.').unwrap_or(0xE8);
                self.place_char(period);
                self.engine.state = TextState::Done;
                TextResult::Done
            }
            _ => TextResult::Continue,
        }
    }

    fn place_char(&mut self, byte: u8) {
        self.tilemap.set(self.engine.cursor, byte);
        self.engine.advance_cursor();
    }

    fn place_name_string(&mut self, name: &[u8]) {
        for &b in name {
            if b == charmap::CHAR_TERMINATOR {
                break;
            }
            self.place_char(b);
        }
    }

    fn process_text_until_terminator(&mut self, stream: &mut TextStream) {
        loop {
            match stream.peek() {
                None | Some(0x50) => break,
                Some(b) if b <= 0x17 => break,
                Some(b) if inline_control_chars::is_inline_control(b) => break,
                Some(_) => {
                    let b = stream.read().unwrap();
                    self.place_char(b);
                }
            }
        }
    }

    fn scroll_text_up(&mut self) {
        // Matches ScrollTextUpOneLine — called twice in original
        self.tilemap.scroll_lines_up(14, 3);
        self.tilemap.scroll_lines_up(14, 3);
    }

    pub fn process_paragraph_resume(&mut self) {
        let arrow_pos = self.engine.text_box.arrow_coord();
        self.tilemap.set(arrow_pos, charmap::CHAR_SPACE);
        self.tilemap.clear_area(TileCoord::new(1, 13), 18, 4);
        self.engine.move_to_coord(TileCoord::new(1, 14));
        self.engine.state = TextState::Printing;
    }

    pub fn process_page_resume(&mut self) {
        let arrow_pos = self.engine.text_box.arrow_coord();
        self.tilemap.set(arrow_pos, charmap::CHAR_SPACE);
        self.tilemap.clear_area(TileCoord::new(1, 10), 18, 7);
        self.engine.move_to_coord(TileCoord::new(1, 11));
        self.engine.state = TextState::Printing;
    }

    pub fn process_scroll_resume(&mut self) {
        let arrow_pos = self.engine.text_box.arrow_coord();
        self.tilemap.set(arrow_pos, charmap::CHAR_SPACE);
        self.scroll_text_up();
        self.engine.move_to_coord(TileCoord::new(1, 16));
        self.engine.state = TextState::Printing;
    }

    pub fn process_prompt_resume(&mut self) {
        let arrow_pos = self.engine.text_box.arrow_coord();
        self.tilemap.set(arrow_pos, charmap::CHAR_SPACE);
        self.engine.state = TextState::Printing;
    }
}
