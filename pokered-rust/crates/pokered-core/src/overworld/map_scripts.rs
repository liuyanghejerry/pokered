use pokered_data::maps::MapId;
use pokered_data::trainer_headers::{get_trainer_headers, TrainerHeaderData};

use super::event_flags::EventFlags;
use super::script_engine::{CoordEvent, MapScriptState};
use super::trainer_engine::{
    advance_trainer_battle, check_all_trainers, TrainerEncounter, TrainerHeader, TrainerPosition,
};

#[derive(Debug, Clone)]
pub struct MapScriptContext {
    pub script_state: MapScriptState,
    pub coord_events: Vec<CoordEvent>,
    pub active_trainer_encounter: Option<TrainerEncounter>,
}

impl MapScriptContext {
    pub fn new(map: MapId) -> Self {
        Self {
            script_state: MapScriptState::new(map),
            coord_events: Vec::new(),
            active_trainer_encounter: None,
        }
    }

    pub fn with_coord_events(mut self, events: Vec<CoordEvent>) -> Self {
        self.coord_events = events;
        self
    }

    pub fn check_coord_trigger(&self, player_x: u8, player_y: u8) -> Option<u8> {
        super::script_engine::check_coord_events(&self.coord_events, player_x, player_y)
    }

    pub fn check_trainers(
        &self,
        flags: &EventFlags,
        trainer_positions: &[TrainerPosition],
        player_x: u8,
        player_y: u8,
    ) -> Option<usize> {
        let headers = get_trainer_headers(self.script_state.map);
        let our_headers: Vec<TrainerHeader> = headers
            .iter()
            .map(|h| TrainerHeader {
                event_flag: h.event_flag,
                sight_range: h.sight_range,
                before_battle_text_id: 0,
                end_battle_text_id: 0,
                after_battle_text_id: 0,
            })
            .collect();
        check_all_trainers(&our_headers, flags, trainer_positions, player_x, player_y)
    }

    pub fn start_trainer_battle(&mut self, trainer_index: u8, npc_index: u8) {
        self.active_trainer_encounter = Some(TrainerEncounter::new(
            self.script_state.map,
            trainer_index,
            npc_index,
        ));
    }

    pub fn advance_active_encounter(
        &mut self,
    ) -> Option<super::trainer_engine::TrainerBattleState> {
        self.active_trainer_encounter
            .as_mut()
            .map(|e| advance_trainer_battle(e))
    }

    pub fn is_encounter_active(&self) -> bool {
        self.active_trainer_encounter.is_some()
    }

    pub fn clear_encounter(&mut self) {
        self.active_trainer_encounter = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapScriptType {
    NoScripts,
    SimpleTextOnly,
    TrainerMap,
    StorySequence,
}

pub fn classify_map_script_type(map: MapId) -> MapScriptType {
    let headers = get_trainer_headers(map);

    match map {
        MapId::PalletTown
        | MapId::OaksLab
        | MapId::CeruleanCity
        | MapId::LavenderTown
        | MapId::SaffronCity
        | MapId::CinnabarIsland
        | MapId::PokemonTower7F
        | MapId::SilphCo7F
        | MapId::SilphCo11F => MapScriptType::StorySequence,

        _ if !headers.is_empty() => MapScriptType::TrainerMap,

        MapId::Route1
        | MapId::ViridianPokecenter
        | MapId::ViridianMart
        | MapId::ViridianCity
        | MapId::PewterCity => MapScriptType::SimpleTextOnly,

        _ => MapScriptType::NoScripts,
    }
}

pub fn get_map_trainer_count(map: MapId) -> usize {
    get_trainer_headers(map).len()
}

pub fn get_map_trainer_header(map: MapId, index: usize) -> Option<&'static TrainerHeaderData> {
    get_trainer_headers(map).get(index)
}
