use pokered_core::data::maps::MapId;
use pokered_core::data::sprites::SpriteId;
use pokered_core::data::{blockset_data, map_blocks, map_data::MAP_HEADER_DATA};
use pokered_core::overworld::{Direction, MovementState, OverworldScreen};
use pokered_renderer::embedded_font::draw_text;
use pokered_renderer::palette::{Palette, GRAYSCALE_PALETTE};
use pokered_renderer::resource::ResourceManager;
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use super::{blit_single_tile, blit_single_tile_flipped, draw_text_box};

pub fn draw_overworld(
    screen: &OverworldScreen,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    // Sprite palette: color 0 is transparent (matches Game Boy OBP0/OBP1 behavior).
    let sprite_pal = Palette::new([
        Rgba::TRANSPARENT,
        Rgba::rgb(0xAA, 0xAA, 0xAA),
        Rgba::rgb(0x55, 0x55, 0x55),
        Rgba::rgb(0x00, 0x00, 0x00),
    ]);

    // Player coordinates are in step units (16px, 2 per block).
    // The renderer works in tile units (8px, 4 per block).
    // Multiply by 2 to convert steps → tiles.
    let player_tx = screen.state.player.x as i32 * 2;
    let player_ty = screen.state.player.y as i32 * 2;
    let screen_center_tx = 9_i32;
    let screen_center_ty = 8_i32;
    let view_origin_tx = player_tx - screen_center_tx;
    let view_origin_ty = player_ty - screen_center_ty;

    if let Some(ref mut rm) = res {
        let current_map: MapId = screen.state.current_map;
        let map_header = &MAP_HEADER_DATA[current_map as usize];
        let tileset_id = map_header.tileset;
        let border_block = map_header.border_block;
        let tileset_name = tileset_id.tileset_name();

        if let Ok(cached) = rm.load_tileset(tileset_name) {
            let ts = cached.tileset.clone();

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

        // Player sprite: 16×96 sheet = 6 frames of 16×16
        // Frame layout: DownStand=0, UpStand=1, LeftStand=2, DownWalk=3, UpWalk=4, LeftWalk=5
        // Right uses Left frames with horizontal flip
        if let Ok(cached) = rm.load_sprite("red") {
            let ts = cached.tileset.clone();

            let (frame, flip_h) = if screen.state.player.movement_state == MovementState::Walking {
                // Original game uses separate anim counter updated every 4 frames
                // walk_counter goes 8->0, so split into two halves: 8-5 (walk) and 4-1 (stand)
                let walk_frame = screen.state.walk_counter > 4;
                match screen.state.player.facing {
                    Direction::Down => (if walk_frame { 3 } else { 0 }, false),
                    Direction::Up => (if walk_frame { 4 } else { 1 }, false),
                    Direction::Left => (if walk_frame { 5 } else { 2 }, false),
                    Direction::Right => (if walk_frame { 5 } else { 2 }, true),
                }
            } else {
                match screen.state.player.facing {
                    Direction::Down => (0, false),
                    Direction::Up => (1, false),
                    Direction::Left => (2, false),
                    Direction::Right => (2, true),
                }
            };

            let base_tile = frame * 4;
            let tpr = cached.source_size.0 / TILE_SIZE;

            let player_px_x = screen_center_tx as u32 * TILE_SIZE;
            let player_px_y = screen_center_ty as u32 * TILE_SIZE;

            for row in 0..2_u32 {
                for col in 0..2_u32 {
                    let src_col = if flip_h { 1 - col } else { col };
                    let tile_idx = base_tile + (row as usize * tpr as usize) + src_col as usize;
                    if tile_idx >= ts.len() {
                        continue;
                    }

                    blit_single_tile_flipped(
                        fb,
                        &ts,
                        tile_idx,
                        player_px_x + col * TILE_SIZE,
                        player_px_y + row * TILE_SIZE,
                        &sprite_pal,
                        flip_h,
                    );
                }
            }
        }

        // Render NPCs
        if let Some(ref map_data) = screen.map_data {
            for npc in &map_data.npcs {
                let sprite_id = match SpriteId::from_u8(npc.sprite_id) {
                    Some(id) => id,
                    None => continue,
                };

                let sprite_name = sprite_id.sprite_name();
                if let Ok(cached) = rm.load_sprite(sprite_name) {
                    let ts = cached.tileset.clone();
                    let num_frames = (cached.source_size.1 / TILE_SIZE) as usize;

                    // NPC sprite layout (16x96 = 6 frames or 16x48 = 3 frames):
                    // 6-frame: [DownStand, UpStand, RightStand, DownWalk, UpWalk, RightWalk]
                    // 3-frame: [DownStand, DownWalk1, DownWalk2] (single direction)
                    let (frame, flip_h) = if num_frames >= 6 {
                        match npc.facing {
                            Direction::Down => (0, false),
                            Direction::Up => (1, false),
                            Direction::Right => (2, false),
                            Direction::Left => (2, true),
                        }
                    } else {
                        (0, false)
                    };

                    let base_tile = frame * 4;
                    let tpr = cached.source_size.0 / TILE_SIZE;

                    // NPC coordinates are in step units; convert to tile units (* 2)
                    let npc_screen_x = (npc.x as i32 * 2 - view_origin_tx) as i32;
                    let npc_screen_y = (npc.y as i32 * 2 - view_origin_ty) as i32;

                    if npc_screen_x < 0
                        || npc_screen_x >= 20
                        || npc_screen_y < 0
                        || npc_screen_y >= 18
                    {
                        continue;
                    }

                    let npc_px_x = npc_screen_x as u32 * TILE_SIZE;
                    let npc_px_y = npc_screen_y as u32 * TILE_SIZE;

                    for row in 0..2_u32 {
                        for col in 0..2_u32 {
                            let src_col = if flip_h { 1 - col } else { col };
                            let tile_idx =
                                base_tile + (row as usize * tpr as usize) + src_col as usize;
                            if tile_idx >= ts.len() {
                                continue;
                            }

                            blit_single_tile_flipped(
                                fb,
                                &ts,
                                tile_idx,
                                npc_px_x + col * TILE_SIZE,
                                npc_px_y + row * TILE_SIZE,
                                &sprite_pal,
                                flip_h,
                            );
                        }
                    }
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

    if screen.map_name_timer > 0 && !screen.state.current_map.is_indoor() {
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

    let fade_progress = screen.warp_fade_progress();
    if fade_progress > 0.0 {
        let darkness = (fade_progress.clamp(0.0, 1.0) * 255.0) as u8;
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if let Some(pixel) = fb.get_pixel(x, y) {
                    let r = ((pixel.0[0] as u16) * (255 - darkness as u16) / 255) as u8;
                    let g = ((pixel.0[1] as u16) * (255 - darkness as u16) / 255) as u8;
                    let b = ((pixel.0[2] as u16) * (255 - darkness as u16) / 255) as u8;
                    fb.set_pixel(x, y, Rgba::rgb(r, g, b));
                }
            }
        }
    }
}
