use pokered_data::maps::MapId;
use pokered_script::{CommandResult, ScriptCommand};

use super::npc_movement::NpcRuntimeState;
use super::{BedroomDialogue, DialoguePage, Direction};

pub use pokered_script::config::{CoordEventBinding, NpcBinding, SignBinding};

#[derive(Debug, Clone)]
pub enum ScriptEffect {
    ShowDialogue {
        text: String,
    },
    ShowChoice {
        options: Vec<String>,
    },
    GiveItem {
        item_id: String,
        quantity: u8,
    },
    TakeItem {
        item_id: String,
        quantity: u8,
    },
    GivePokemon {
        species: String,
        level: u8,
    },
    ShowObject {
        object_index: u8,
    },
    HideObject {
        object_index: u8,
    },
    ShowObjectByName {
        toggle_id: String,
    },
    HideObjectByName {
        toggle_id: String,
    },
    MoveNpc {
        npc_id: String,
        path: Vec<(u8, u8)>,
        started: bool,
    },
    StartNpcMove {
        npc_id: String,
        path: Vec<(u8, u8)>,
    },
    AwaitNpcMove {
        npc_id: String,
    },
    MovePlayer {
        path: Vec<(u8, u8)>,
        started: bool,
    },
    FaceNpc {
        npc_id: String,
        direction: Direction,
    },
    FacePlayer {
        direction: Direction,
    },
    PlayMusic {
        music_id: String,
    },
    PlaySound {
        sound_id: String,
    },
    StopMusic,
    FadeOutMusic,
    StartBattle {
        trainer_id: String,
    },
    Delay {
        frames: u16,
        frames_remaining: u16,
    },
    WarpTo {
        map: String,
        x: u8,
        y: u8,
    },
    Heal,
    FadeScreen {
        fade_type: String,
    },
    SetJoyIgnore {
        mask: u8,
    },
    ClearJoyIgnore,
    Immediate {
        result: CommandResult,
    },
}

pub fn parse_direction(s: &str) -> Option<Direction> {
    match s.to_lowercase().as_str() {
        "up" | "north" => Some(Direction::Up),
        "down" | "south" => Some(Direction::Down),
        "left" | "west" => Some(Direction::Left),
        "right" | "east" => Some(Direction::Right),
        _ => None,
    }
}

pub fn dispatch_command(cmd: &ScriptCommand) -> ScriptEffect {
    match cmd {
        ScriptCommand::ShowText { text } => ScriptEffect::ShowDialogue { text: text.clone() },
        ScriptCommand::ShowChoice { options } => ScriptEffect::ShowChoice {
            options: options.clone(),
        },
        ScriptCommand::GiveItem { item_id, quantity } => ScriptEffect::GiveItem {
            item_id: item_id.clone(),
            quantity: *quantity,
        },
        ScriptCommand::TakeItem { item_id, quantity } => ScriptEffect::TakeItem {
            item_id: item_id.clone(),
            quantity: *quantity,
        },
        ScriptCommand::GivePokemon { species, level } => ScriptEffect::GivePokemon {
            species: species.clone(),
            level: *level,
        },
        ScriptCommand::ShowObject { object_index } => ScriptEffect::ShowObject {
            object_index: *object_index,
        },
        ScriptCommand::HideObject { object_index } => ScriptEffect::HideObject {
            object_index: *object_index,
        },
        ScriptCommand::ShowObjectByName { toggle_id } => ScriptEffect::ShowObjectByName {
            toggle_id: toggle_id.clone(),
        },
        ScriptCommand::HideObjectByName { toggle_id } => ScriptEffect::HideObjectByName {
            toggle_id: toggle_id.clone(),
        },
        ScriptCommand::MoveNpc { npc_id, path } => ScriptEffect::MoveNpc {
            npc_id: npc_id.clone(),
            path: path.clone(),
            started: false,
        },
        ScriptCommand::StartNpcMove { npc_id, path } => ScriptEffect::StartNpcMove {
            npc_id: npc_id.clone(),
            path: path.clone(),
        },
        ScriptCommand::AwaitNpcMove { npc_id } => ScriptEffect::AwaitNpcMove {
            npc_id: npc_id.clone(),
        },
        ScriptCommand::MovePlayer { path } => ScriptEffect::MovePlayer {
            path: path.clone(),
            started: false,
        },
        ScriptCommand::FaceNpc { npc_id, direction } => ScriptEffect::FaceNpc {
            npc_id: npc_id.clone(),
            direction: parse_direction(direction).unwrap_or(Direction::Down),
        },
        ScriptCommand::FacePlayer { direction } => ScriptEffect::FacePlayer {
            direction: parse_direction(direction).unwrap_or(Direction::Down),
        },
        ScriptCommand::PlayMusic { music_id } => ScriptEffect::PlayMusic {
            music_id: music_id.clone(),
        },
        ScriptCommand::PlaySound { sound_id } => ScriptEffect::PlaySound {
            sound_id: sound_id.clone(),
        },
        ScriptCommand::StopMusic => ScriptEffect::StopMusic,
        ScriptCommand::FadeOutMusic => ScriptEffect::FadeOutMusic,
        ScriptCommand::StartBattle { trainer_id } => ScriptEffect::StartBattle {
            trainer_id: trainer_id.clone(),
        },
        ScriptCommand::Delay { frames } => ScriptEffect::Delay {
            frames: *frames,
            frames_remaining: *frames,
        },
        ScriptCommand::WarpTo { map, x, y } => ScriptEffect::WarpTo {
            map: map.clone(),
            x: *x,
            y: *y,
        },
        ScriptCommand::Heal => ScriptEffect::Heal,
        ScriptCommand::FadeScreen { fade_type } => ScriptEffect::FadeScreen {
            fade_type: fade_type.clone(),
        },
        ScriptCommand::SetJoyIgnore { mask } => ScriptEffect::SetJoyIgnore { mask: *mask },
        ScriptCommand::ClearJoyIgnore => ScriptEffect::ClearJoyIgnore,
        // Sync flag ops never reach dispatch — defensive fallback.
        ScriptCommand::SetFlag { .. }
        | ScriptCommand::ResetFlag { .. }
        | ScriptCommand::CheckFlag { .. } => ScriptEffect::Immediate {
            result: CommandResult::Void,
        },
    }
}

pub fn text_to_dialogue(text: &str) -> BedroomDialogue {
    let lines: Vec<&str> = text.lines().collect();
    let mut pages = Vec::new();

    if lines.is_empty() {
        pages.push(DialoguePage {
            line1: Box::leak(text.to_string().into_boxed_str()),
            line2: "",
        });
    } else {
        let mut i = 0;
        while i < lines.len() {
            let line1 = Box::leak(lines[i].to_string().into_boxed_str()) as &'static str;
            let line2 = if i + 1 < lines.len() {
                i += 1;
                Box::leak(lines[i].to_string().into_boxed_str()) as &'static str
            } else {
                ""
            };
            pages.push(DialoguePage { line1, line2 });
            i += 1;
        }
    }

    BedroomDialogue::from_pages(pages)
}

pub fn map_id_to_script_key(map_id: MapId) -> String {
    format!("{:?}", map_id)
}

pub fn find_npc_index_by_id(npcs: &[NpcRuntimeState], npc_id: &str) -> Option<usize> {
    if let Ok(idx) = npc_id.parse::<u8>() {
        return npcs.iter().position(|n| n.npc_index == idx);
    }
    None
}
