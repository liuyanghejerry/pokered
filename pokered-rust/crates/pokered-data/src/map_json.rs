//! JSON-serializable map data types.
//!
//! These types define the schema for `map.json` files in `maps/{MapName}/`.
//! Used by both the generator tool (serialization) and the runtime loader (deserialization).

use serde::{Deserialize, Serialize};

/// Complete map data as stored in `map.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapJson {
    /// Numeric map ID (0x00-0xF7)
    pub id: u8,
    /// Map name matching MapId variant (e.g. "PalletTown")
    pub name: String,
    /// Map header data
    pub header: MapHeaderJson,
    /// Cardinal connections to adjacent maps
    #[serde(default)]
    pub connections: ConnectionsJson,
    /// Warp points (doors, stairs, etc.)
    #[serde(default)]
    pub warps: Vec<WarpJson>,
    /// NPC definitions
    #[serde(default)]
    pub npcs: Vec<NpcJson>,
    /// Sign definitions
    #[serde(default)]
    pub signs: Vec<SignJson>,
    /// Dialog text for NPCs and signs
    #[serde(default)]
    pub text: MapTextJson,
    /// Wild encounter data (per game version)
    #[serde(default)]
    pub wild: Option<WildDataJson>,
}

/// Map header — tileset, music, dimensions, connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapHeaderJson {
    /// Tileset name (e.g. "Overworld", "House")
    pub tileset: String,
    /// Music track name (e.g. "PalletTown", "Cities1")
    pub music: String,
    /// Bitfield: bit3=north, bit2=south, bit1=west, bit0=east
    pub connection_flags: u8,
    /// Map width in blocks
    pub width: u8,
    /// Map height in blocks
    pub height: u8,
    /// Border block ID
    pub border_block: u8,
}

/// Cardinal connections to adjacent maps.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionsJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub north: Option<ConnectionEntryJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub south: Option<ConnectionEntryJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub west: Option<ConnectionEntryJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub east: Option<ConnectionEntryJson>,
}

/// A single map connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionEntryJson {
    /// Target map name (e.g. "Route1")
    pub target_map: String,
    /// Offset for alignment
    pub offset: i8,
}

/// A warp point (door, stairs, cave entrance, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarpJson {
    pub x: u8,
    pub y: u8,
    /// Destination map name, or null for LAST_MAP / dynamic destination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dest_map: Option<String>,
    pub dest_warp_id: u8,
}

/// An NPC definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcJson {
    pub sprite_id: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite_name: Option<String>,
    pub x: u8,
    pub y: u8,
    /// Movement type: "Stationary", "Wander", "FixedPath", "FacePlayer"
    pub movement: String,
    /// Facing direction: "Down", "Up", "Left", "Right"
    pub facing: String,
    pub range: u8,
    pub text_id: u8,
    pub is_trainer: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trainer_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trainer_set: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<u8>,
}

/// A sign definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignJson {
    pub x: u8,
    pub y: u8,
    pub text_id: u8,
}

/// Dialog text for all NPCs and signs on a map.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapTextJson {
    /// NPC dialog: key = text_id (string), value = array of text pages
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub npc: std::collections::HashMap<String, Vec<TextPageJson>>,
    /// Sign dialog: key = text_id (string), value = array of text pages
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub sign: std::collections::HashMap<String, Vec<TextPageJson>>,
}

/// A single dialog page (two lines displayed in the text box).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPageJson {
    pub line1: String,
    pub line2: String,
}

/// Wild encounter data for both game versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildDataJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red: Option<VersionWildJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blue: Option<VersionWildJson>,
}

/// Wild encounters for a single version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionWildJson {
    pub grass: WildEncounterTableJson,
    pub water: WildEncounterTableJson,
}

/// A wild encounter table (rate + 10 slots).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WildEncounterTableJson {
    pub encounter_rate: u8,
    pub mons: Vec<WildMonJson>,
}

/// A single wild encounter slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildMonJson {
    pub level: u8,
    pub species: String,
}
