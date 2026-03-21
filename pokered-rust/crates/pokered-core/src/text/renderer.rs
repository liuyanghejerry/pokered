use pokered_data::charmap;

use super::processor::{TextProcessor, TextStream};
use super::{NameBuffers, TextEngine, TextResult, TextState, TilemapBuffer};

pub struct TextRenderer {
    pub engine: TextEngine,
    pub tilemap: TilemapBuffer,
    pub names: NameBuffers,
    pub stream: Option<TextStream>,
    pub auto_advance: bool,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            engine: TextEngine::new(),
            tilemap: TilemapBuffer::default(),
            names: NameBuffers::default(),
            stream: None,
            auto_advance: false,
        }
    }

    pub fn begin_text(&mut self, data: Vec<u8>) {
        let text_box = super::TextBox::standard_dialog();
        self.tilemap.draw_box_border(&text_box);
        self.engine.start(text_box);
        self.stream = Some(TextStream::new(data));
    }

    pub fn begin_text_at(&mut self, data: Vec<u8>, text_box: super::TextBox) {
        self.tilemap.draw_box_border(&text_box);
        self.engine.start(text_box);
        self.stream = Some(TextStream::new(data));
    }

    pub fn tick(&mut self, button_pressed: bool) -> TextResult {
        match self.engine.state {
            TextState::Idle => TextResult::Done,

            TextState::Printing => {
                if let Some(ref mut stream) = self.stream {
                    let names = unsafe { &*(&self.names as *const NameBuffers) };
                    let mut proc = TextProcessor::new(&mut self.engine, &mut self.tilemap, names);
                    proc.process_next_byte(stream)
                } else {
                    self.engine.state = TextState::Done;
                    TextResult::Done
                }
            }

            TextState::WaitButton => {
                if button_pressed || self.auto_advance {
                    let names = unsafe { &*(&self.names as *const NameBuffers) };
                    let mut proc = TextProcessor::new(&mut self.engine, &mut self.tilemap, names);
                    proc.process_prompt_resume();
                    TextResult::Continue
                } else {
                    TextResult::WaitForButton
                }
            }

            TextState::WaitScroll => {
                if button_pressed || self.auto_advance {
                    let names = unsafe { &*(&self.names as *const NameBuffers) };
                    let mut proc = TextProcessor::new(&mut self.engine, &mut self.tilemap, names);
                    proc.process_scroll_resume();
                    TextResult::Continue
                } else {
                    TextResult::WaitForScroll
                }
            }

            TextState::Paused { .. } => {
                self.engine.tick_pause();
                TextResult::Continue
            }

            TextState::AsmCallback => TextResult::StartAsm,

            TextState::Done => TextResult::Done,
        }
    }

    pub fn run_to_completion(&mut self) -> Vec<TextResult> {
        let mut results = Vec::new();
        self.auto_advance = true;
        loop {
            let result = self.tick(true);
            results.push(result.clone());
            match result {
                TextResult::Done | TextResult::StartAsm => break,
                _ => {}
            }
        }
        self.auto_advance = false;
        results
    }

    pub fn is_done(&self) -> bool {
        self.engine.is_done()
    }

    pub fn read_tilemap_text(&self, x: u8, y: u8, len: u8) -> String {
        let mut result = String::new();
        for i in 0..len {
            let tile = self.tilemap.get(super::TileCoord::new(x + i, y));
            if tile == charmap::CHAR_TERMINATOR {
                break;
            }
            if let Some(s) = charmap::decode_char(tile) {
                result.push_str(s);
            }
        }
        result
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
