//! Export map data to JSON for the map viewer tool
//!
//! Usage: cargo run --bin export_map_data > tools/map_data.json

use serde::Serialize;

use pokered_data::blockset_data;
use pokered_data::collision;
use pokered_data::map_blocks;
use pokered_data::map_data::MAP_HEADER_DATA;
use pokered_data::map_objects;
use pokered_data::map_text_data;
use pokered_data::maps::MapId;
use pokered_data::npc_data;
use pokered_data::sign_data;
use pokered_data::sprites::SpriteId;
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
    sprite_name: String,
    x: u8,
    y: u8,
    movement: String,
    facing: String,
    range: u8,
    text_id: u8,
    is_trainer: bool,
    trainer_class: Option<String>,
    trainer_set: Option<u8>,
    item_id: Option<u8>,
    text_pages: Vec<TextPageExport>,
}

#[derive(serde::Serialize)]
struct SignExport {
    x: u8,
    y: u8,
    text_id: u8,
    text_pages: Vec<TextPageExport>,
}

#[derive(serde::Serialize)]
struct TextPageExport {
    line1: String,
    line2: String,
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

fn movement_str(movement: u8) -> String {
    match movement {
        0 => "Stationary".to_string(),
        1 => "Wander".to_string(),
        2 => "FixedPath".to_string(),
        3 => "FacePlayer".to_string(),
        _ => format!("Unknown({})", movement),
    }
}

fn trainer_class_name(class_id: u8) -> String {
    match class_id {
        0 => "Nobody".to_string(),
        1 => "Youngster".to_string(),
        2 => "BugCatcher".to_string(),
        3 => "Lass".to_string(),
        4 => "Sailor".to_string(),
        5 => "JrTrainerM".to_string(),
        6 => "JrTrainerF".to_string(),
        7 => "Pokemaniac".to_string(),
        8 => "SuperNerd".to_string(),
        9 => "Hiker".to_string(),
        10 => "Biker".to_string(),
        11 => "Burglar".to_string(),
        12 => "Engineer".to_string(),
        14 => "Fisher".to_string(),
        15 => "Swimmer".to_string(),
        16 => "CueBall".to_string(),
        17 => "Gambler".to_string(),
        18 => "Beauty".to_string(),
        19 => "Psychic".to_string(),
        20 => "Rocker".to_string(),
        21 => "Juggler".to_string(),
        22 => "Tamer".to_string(),
        23 => "BirdKeeper".to_string(),
        24 => "Blackbelt".to_string(),
        25 => "Rival1".to_string(),
        26 => "ProfOak".to_string(),
        27 => "Chief".to_string(),
        28 => "Scientist".to_string(),
        29 => "Giovanni".to_string(),
        30 => "Rocket".to_string(),
        31 => "CooltrainerM".to_string(),
        32 => "CooltrainerF".to_string(),
        33 => "Bruno".to_string(),
        34 => "Brock".to_string(),
        35 => "Misty".to_string(),
        36 => "LtSurge".to_string(),
        37 => "Erika".to_string(),
        38 => "Koga".to_string(),
        39 => "Blaine".to_string(),
        40 => "Sabrina".to_string(),
        41 => "Gentleman".to_string(),
        42 => "Rival2".to_string(),
        43 => "Rival3".to_string(),
        44 => "Lorelei".to_string(),
        45 => "Channeler".to_string(),
        46 => "Agatha".to_string(),
        47 => "Lance".to_string(),
        _ => format!("Unknown({})", class_id),
    }
}

fn sprite_name_str(sprite_id: u8) -> String {
    SpriteId::from_u8(sprite_id)
        .map(|s| format!("{:?}", s))
        .unwrap_or_else(|| format!("Unknown({})", sprite_id))
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

            let npcs: Vec<NpcExport> = npc_data::get_map_npcs(map_id)
                .iter()
                .map(|n| {
                    let dialog = map_text_data::get_npc_text(map_id, n.text_id);
                    let text_pages = dialog
                        .iter()
                        .map(|p| TextPageExport {
                            line1: p.line1.to_string(),
                            line2: p.line2.to_string(),
                        })
                        .collect();
                    NpcExport {
                        sprite_id: n.sprite_id,
                        sprite_name: sprite_name_str(n.sprite_id),
                        x: n.x,
                        y: n.y,
                        movement: movement_str(n.movement.0),
                        facing: facing_str(n.facing.0),
                        range: n.range,
                        text_id: n.text_id,
                        is_trainer: n.is_trainer,
                        trainer_class: if n.is_trainer {
                            Some(trainer_class_name(n.trainer_class))
                        } else {
                            None
                        },
                        trainer_set: if n.is_trainer {
                            Some(n.trainer_set)
                        } else {
                            None
                        },
                        item_id: if n.item_id > 0 { Some(n.item_id) } else { None },
                        text_pages,
                    }
                })
                .collect();

            let signs: Vec<SignExport> = sign_data::get_map_signs(map_id)
                .iter()
                .map(|s| {
                    let dialog = map_text_data::get_sign_text(map_id, s.text_id);
                    let text_pages = dialog
                        .iter()
                        .map(|p| TextPageExport {
                            line1: p.line1.to_string(),
                            line2: p.line2.to_string(),
                        })
                        .collect();
                    SignExport {
                        x: s.x,
                        y: s.y,
                        text_id: s.text_id,
                        text_pages,
                    }
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
