use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptCommand {
    ShowText { text: String },
    ShowChoice { options: Vec<String> },
    GiveItem { item_id: String, quantity: u8 },
    GivePokemon { species: String, level: u8 },
    TakeItem { item_id: String, quantity: u8 },
    SetFlag { flag: String },
    ResetFlag { flag: String },
    CheckFlag { flag: String },
    ShowObject { object_index: u8 },
    HideObject { object_index: u8 },
    MoveNpc { npc_id: String, path: Vec<(u8, u8)> },
    FaceNpc { npc_id: String, direction: String },
    FacePlayer { direction: String },
    PlayMusic { music_id: String },
    PlaySound { sound_id: String },
    StopMusic,
    FadeOutMusic,
    StartBattle { trainer_id: String },
    Delay { frames: u16 },
    WarpTo { map: String, x: u8, y: u8 },
    Heal,
    FadeScreen { fade_type: String },
    SetMapScript { script_index: u8 },
    SetJoyIgnore { mask: u8 },
    ClearJoyIgnore,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    Void,
    Bool(bool),
    Number(f64),
    Text(String),
}
