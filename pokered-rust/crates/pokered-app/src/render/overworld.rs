use pokered_core::data::maps::MapId;
use pokered_core::data::{blockset_data, map_blocks, map_data::MAP_HEADER_DATA};
use pokered_core::overworld::{Direction, MovementState, OverworldScreen};
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::palette::GRAYSCALE_PALETTE;
use pokered_renderer::resource::ResourceManager;
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_WIDTH, TILE_SIZE};

use super::{blit_single_tile, draw_text_box};

pub fn draw_overworld(
    screen: &OverworldScreen,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    let player_tx = screen.state.player.x as i32;
    let player_ty = screen.state.player.y as i32;
    let screen_center_tx = 9_i32;
    let screen_center_ty = 8_i32;

    if let Some(ref mut rm) = res {
        let current_map: MapId = screen.state.current_map;
        let map_header = &MAP_HEADER_DATA[current_map as usize];
        let tileset_id = map_header.tileset;
        let border_block = map_header.border_block;
        let tileset_name = tileset_id.tileset_name();

        if let Ok(cached) = rm.load_tileset(tileset_name) {
            let ts = cached.tileset.clone();

            let view_origin_tx = player_tx - screen_center_tx;
            let view_origin_ty = player_ty - screen_center_ty;

            let (map_w, map_h) = current_map.dimensions();
            let blk = map_blocks::block_data_for_map(current_map);

            for sy in 0..18_i32 {
                for sx in 0..20_i32 {
                    let world_tx = view_origin_tx + sx;
                    let world_ty = view_origin_ty + sy;

                    let bx = world_tx.div_euclid(4);
                    let by = world_ty.div_euclid(4);
                    let sub_x = world_tx.rem_euclid(4) as usize;
                    let sub_y = world_ty.rem_euclid(4) as usize;

                    let block_id = if bx >= 0
                        && by >= 0
                        && (bx as u8) < map_w
                        && (by as u8) < map_h
                        && !blk.is_empty()
                    {
                        blk[(by as usize) * (map_w as usize) + (bx as usize)]
                    } else {
                        border_block
                    };

                    let tile_idx = blockset_data::block_tiles(tileset_id, block_id)
                        .map(|t| t[sub_y * 4 + sub_x] as usize)
                        .unwrap_or(0)
                        .min(ts.len().saturating_sub(1));

                    let px = (sx as u32) * TILE_SIZE;
                    let py = (sy as u32) * TILE_SIZE;
                    blit_single_tile(fb, &ts, tile_idx, px, py, pal);
                }
            }
        }

        // Player sprite: 16×96 sheet = 6 frames of 16×16 (2×2 tiles each)
        // Frame layout: Down[0,1], Right[2,3], Left[4,5]
        // Up direction uses Down frames (no separate up-facing sprite)
        if let Ok(cached) = rm.load_sprite("red") {
            let ts = cached.tileset.clone();
            let mut frame = match screen.state.player.facing {
                Direction::Down => 0,
                Direction::Up => 0,
                Direction::Right => 2,
                Direction::Left => 4,
            };

            if screen.state.player.movement_state == MovementState::Walking {
                if screen.state.walk_counter % 2 == 0 {
                    frame += 1;
                }
            }

            let base_tile = frame * 4;
            let tpr = cached.source_size.0 / TILE_SIZE;

            let player_px_x = screen_center_tx as u32 * TILE_SIZE;
            let player_px_y = screen_center_ty as u32 * TILE_SIZE;

            for row in 0..2_u32 {
                for col in 0..2_u32 {
                    let tile_idx = base_tile + (row as usize * tpr as usize) + col as usize;
                    if tile_idx >= ts.len() {
                        continue;
                    }

                    blit_single_tile(
                        fb,
                        &ts,
                        tile_idx,
                        player_px_x + col * TILE_SIZE,
                        player_px_y + row * TILE_SIZE,
                        pal,
                    );
                }
            }
        }
    }

    if let Some(ref dlg) = screen.pending_dialogue {
        if let Some(page) = dlg.current() {
            let text_box_x = 0_u32;
            let text_box_y = 12 * TILE_SIZE;
            draw_text_box(fb, text_box_x, text_box_y, 18, 4, Rgba::BLACK);
            draw_text(
                page.line1,
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
            draw_text(
                page.line2,
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE * 3,
                Rgba::BLACK,
                fb,
            );
        }
        return;
    }

    let map_name = format!("{:?}", screen.state.current_map);
    let name_len = map_name.len() as u32;
    let box_w = name_len.max(4) + 2;
    let box_x = (SCREEN_WIDTH.saturating_sub((box_w + 2) * TILE_SIZE)) / 2;
    let box_y = 13 * TILE_SIZE;
    draw_text_box(fb, box_x, box_y, box_w, 2, Rgba::BLACK);
    draw_text(
        &map_name,
        box_x + TILE_SIZE,
        box_y + TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}
