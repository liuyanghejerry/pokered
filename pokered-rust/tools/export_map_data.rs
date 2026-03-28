//! Export map data to JSON for the map viewer tool
//!
//! Usage: cargo run --bin export_map_data > tools/map_data.json

use serde::Serialize;

use pokered_data::blockset_data;
use pokered_data::collision;
use pokered_data::map_blocks;
use pokered_data::map_data::MAP_HEADER_DATA;
use pokered_data::map_objects;
use pokered_data::maps::MapId;
use pokered_data::sign_data;
use pokered_data::tilesets::TilesetId;

#[derive(Serialize)]
struct ExportData {
    maps: Vec<MapExport>,
    blocksets: Vec<BlocksetExport>,
    tileset_names: Vec<(u8, String)>,
}

#[derive(Serialize)]
struct BlocksetExport {
    tileset_id: u8,
    tileset_name: String,
    blocks: Vec<Vec<u8>>,
}

#[derive(Serialize)]
struct MapExport {
    id: u8,
    name: String,
    width: u8,
    height: u8,
    tileset_id: u8,
    tileset_name: String,
    blocks: Vec<u8>,
    warps: Vec<WarpExport>,
    npcs: Vec<NpcExport>,
    signs: Vec<SignExport>,
    passable_tiles: Vec<u8>,
}

#[derive(serde::Serialize)]
struct WarpExport {
    x: u8,
    y: u8,
    dest_map: Option<u8>,
    dest_map_name: Option<String>,
    dest_warp_id: u8,
}

#[derive(serde::Serialize)]
struct NpcExport {
    sprite_id: u8,
    x: u8,
    y: u8,
    facing: String,
}

#[derive(serde::Serialize)]
struct SignExport {
    x: u8,
    y: u8,
    text_id: u8,
}

fn map_name(id: MapId) -> String {
    format!("{:?}", id)
}

fn tileset_name(id: TilesetId) -> String {
    format!("{:?}", id)
}

fn facing_str(facing: u8) -> String {
    match facing {
        0 => "Down".to_string(),
        1 => "Up".to_string(),
        2 => "Left".to_string(),
        3 => "Right".to_string(),
        _ => format!("Unknown({})", facing),
    }
}

fn main() {
    let mut maps: Vec<MapExport> = Vec::new();

    // Export all maps
    for i in 0u8..=0xF7 {
        if let Some(map_id) = MapId::from_u8(i) {
            let header = &MAP_HEADER_DATA[i as usize];
            let blocks = map_blocks::block_data_for_map(map_id);
            let (width, height) = map_id.dimensions();

            let warps: Vec<WarpExport> = map_objects::get_map_warps(map_id)
                .iter()
                .map(|w| WarpExport {
                    x: w.x,
                    y: w.y,
                    dest_map: w.dest_map.map(|m| m as u8),
                    dest_map_name: w.dest_map.map(|m| map_name(m)),
                    dest_warp_id: w.dest_warp_id,
                })
                .collect();

            let npcs: Vec<NpcExport> = Vec::new();

            let signs: Vec<SignExport> = sign_data::get_map_signs(map_id)
                .iter()
                .map(|s| SignExport {
                    x: s.x,
                    y: s.y,
                    text_id: s.text_id,
                })
                .collect();

            let passable_tiles = collision::get_passable_tiles(header.tileset).to_vec();

            maps.push(MapExport {
                id: i,
                name: map_name(map_id),
                width,
                height,
                tileset_id: header.tileset as u8,
                tileset_name: tileset_name(header.tileset),
                blocks: blocks.to_vec(),
                warps,
                npcs,
                signs,
                passable_tiles,
            });
        }
    }

    // Export blocksets (each block = 4x4 tiles = 16 tile IDs)
    let mut blocksets: Vec<BlocksetExport> = Vec::new();
    for tileset_id in 0u8..24 {
        if let Some(tileset) = TilesetId::from_u8(tileset_id) {
            let data = blockset_data::blockset_for_tileset(tileset);
            let num_blocks = data.len() / 16;
            let blocks: Vec<Vec<u8>> = (0..num_blocks)
                .map(|i| data[i * 16..(i + 1) * 16].to_vec())
                .collect();

            blocksets.push(BlocksetExport {
                tileset_id,
                tileset_name: tileset_name(tileset),
                blocks,
            });
        }
    }

    // Tileset names for reference
    let tileset_names: Vec<(u8, String)> = (0u8..24)
        .filter_map(|id| TilesetId::from_u8(id).map(|t| (id, tileset_name(t))))
        .collect();

    let export_data = ExportData {
        maps,
        blocksets,
        tileset_names,
    };

    let output = serde_json::to_string_pretty(&export_data).expect("Failed to serialize");
    println!("{}", output);
}
