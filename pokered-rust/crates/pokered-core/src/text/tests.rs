use pokered_data::charmap;
use pokered_data::text_commands::inline_control_chars;

use super::processor::TextStream;
use super::*;

#[test]
fn text_engine_starts_idle() {
    let engine = TextEngine::new();
    assert!(engine.is_idle());
}

#[test]
fn text_engine_start_sets_printing() {
    let mut engine = TextEngine::new();
    engine.start(TextBox::standard_dialog());
    assert_eq!(engine.state, TextState::Printing);
}

#[test]
fn text_box_standard_dialog() {
    let tb = TextBox::standard_dialog();
    assert_eq!(tb.origin, TileCoord::new(0, 12));
    assert_eq!(tb.width, SCREEN_WIDTH);
    assert_eq!(tb.height, 6);
}

#[test]
fn tile_coord_roundtrip() {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let coord = TileCoord::new(x, y);
            let idx = coord.to_tilemap_index();
            let back = TileCoord::from_tilemap_index(idx);
            assert_eq!(back, coord);
        }
    }
}

#[test]
fn tilemap_draw_box_border() {
    let mut tilemap = TilemapBuffer::default();
    let tb = TextBox::new(TileCoord::new(0, 0), 4, 3);
    tilemap.draw_box_border(&tb);
    assert_eq!(tilemap.get(TileCoord::new(0, 0)), TILE_TOP_LEFT);
    assert_eq!(tilemap.get(TileCoord::new(3, 0)), TILE_TOP_RIGHT);
    assert_eq!(tilemap.get(TileCoord::new(0, 2)), TILE_BOTTOM_LEFT);
    assert_eq!(tilemap.get(TileCoord::new(3, 2)), TILE_BOTTOM_RIGHT);
    assert_eq!(tilemap.get(TileCoord::new(1, 0)), TILE_HORIZONTAL);
    assert_eq!(tilemap.get(TileCoord::new(0, 1)), TILE_VERTICAL);
}

#[test]
fn text_stream_read_basics() {
    let mut stream = TextStream::new(vec![0x01, 0x02, 0x03]);
    assert_eq!(stream.peek(), Some(0x01));
    assert_eq!(stream.read(), Some(0x01));
    assert_eq!(stream.read(), Some(0x02));
    assert_eq!(stream.read(), Some(0x03));
    assert_eq!(stream.read(), None);
    assert!(stream.is_at_end());
}

#[test]
fn text_stream_read_u16_le() {
    let mut stream = TextStream::new(vec![0x34, 0x12]);
    assert_eq!(stream.read_u16_le(), Some(0x1234));
}

#[test]
fn process_simple_text() {
    let mut engine = TextEngine::new();
    let mut tilemap = TilemapBuffer::default();
    let names = NameBuffers::default();
    let text_box = TextBox::standard_dialog();
    tilemap.draw_box_border(&text_box);
    engine.start(text_box);

    let encoded = charmap::encode_string("HI").unwrap();
    // Remove terminator, add DONE control
    let mut data: Vec<u8> = encoded[..encoded.len() - 1].to_vec();
    data.push(inline_control_chars::DONE);

    let mut stream = TextStream::new(data);
    let mut proc = processor::TextProcessor::new(&mut engine, &mut tilemap, &names);

    let r1 = proc.process_next_byte(&mut stream);
    assert_eq!(r1, TextResult::Continue);
    let r2 = proc.process_next_byte(&mut stream);
    assert_eq!(r2, TextResult::Continue);
    let r3 = proc.process_next_byte(&mut stream);
    assert_eq!(r3, TextResult::Done);
}

#[test]
fn process_terminator_ends_text() {
    let mut engine = TextEngine::new();
    let mut tilemap = TilemapBuffer::default();
    let names = NameBuffers::default();
    engine.start(TextBox::standard_dialog());

    let mut stream = TextStream::new(vec![charmap::CHAR_TERMINATOR]);
    let mut proc = processor::TextProcessor::new(&mut engine, &mut tilemap, &names);
    let result = proc.process_next_byte(&mut stream);
    assert_eq!(result, TextResult::Done);
    assert!(engine.is_done());
}

#[test]
fn process_next_line_control() {
    let mut engine = TextEngine::new();
    let mut tilemap = TilemapBuffer::default();
    let names = NameBuffers::default();
    engine.start(TextBox::standard_dialog());

    let start_y = engine.cursor.y;
    let mut stream = TextStream::new(vec![inline_control_chars::NEXT]);
    let mut proc = processor::TextProcessor::new(&mut engine, &mut tilemap, &names);
    proc.process_next_byte(&mut stream);
    assert_eq!(engine.cursor.y, start_y + 2); // double-spaced by default
}

#[test]
fn process_player_name_insertion() {
    let mut engine = TextEngine::new();
    let mut tilemap = TilemapBuffer::default();
    let names = NameBuffers::default(); // player = "RED"
    engine.start(TextBox::standard_dialog());

    let start_x = engine.cursor.x;
    let mut stream = TextStream::new(vec![inline_control_chars::PLAYER]);
    let mut proc = processor::TextProcessor::new(&mut engine, &mut tilemap, &names);
    proc.process_next_byte(&mut stream);
    assert_eq!(engine.cursor.x, start_x + 3); // "RED" = 3 chars
}

#[test]
fn renderer_run_to_completion() {
    let mut renderer = renderer::TextRenderer::new();
    let encoded = charmap::encode_string("OK").unwrap();
    let mut data: Vec<u8> = encoded[..encoded.len() - 1].to_vec();
    data.push(inline_control_chars::DONE);
    renderer.begin_text(data);
    let results = renderer.run_to_completion();
    assert!(renderer.is_done());
    assert!(results.iter().any(|r| *r == TextResult::Done));
}

#[test]
fn renderer_read_tilemap_text() {
    let mut renderer = renderer::TextRenderer::new();
    let encoded = charmap::encode_string("HI").unwrap();
    let mut data: Vec<u8> = encoded[..encoded.len() - 1].to_vec();
    data.push(inline_control_chars::DONE);
    renderer.begin_text(data);
    renderer.run_to_completion();

    let tb = TextBox::standard_dialog();
    let start = tb.text_start_coord();
    let text = renderer.read_tilemap_text(start.x, start.y, 2);
    assert_eq!(text, "HI");
}

#[test]
fn name_buffers_default_player_red() {
    let names = NameBuffers::default();
    let decoded = charmap::decode_string(&names.player_name);
    assert_eq!(decoded, "RED");
}

#[test]
fn name_buffers_default_rival_blue() {
    let names = NameBuffers::default();
    let decoded = charmap::decode_string(&names.rival_name);
    assert_eq!(decoded, "BLUE");
}

#[test]
fn text_engine_tick_pause() {
    let mut engine = TextEngine::new();
    engine.state = TextState::Paused {
        frames_remaining: 2,
    };
    assert!(!engine.tick_pause()); // 2 -> 1
    assert!(engine.tick_pause()); // 1 -> 0, returns true
    assert_eq!(engine.state, TextState::Printing);
}

#[test]
fn tilemap_clear_area() {
    let mut tilemap = TilemapBuffer::default();
    tilemap.set(TileCoord::new(1, 1), 0x80); // 'A'
    tilemap.set(TileCoord::new(2, 1), 0x81); // 'B'
    tilemap.clear_area(TileCoord::new(1, 1), 2, 1);
    assert_eq!(tilemap.get(TileCoord::new(1, 1)), charmap::CHAR_SPACE);
    assert_eq!(tilemap.get(TileCoord::new(2, 1)), charmap::CHAR_SPACE);
}

#[test]
fn process_wait_button_command() {
    let mut engine = TextEngine::new();
    let mut tilemap = TilemapBuffer::default();
    let names = NameBuffers::default();
    engine.start(TextBox::standard_dialog());

    let mut stream = TextStream::new(vec![0x0D]); // TX_WAIT_BUTTON
    let mut proc = processor::TextProcessor::new(&mut engine, &mut tilemap, &names);
    let result = proc.process_next_byte(&mut stream);
    assert_eq!(result, TextResult::WaitForButton);
    assert_eq!(engine.state, TextState::WaitButton);
}
