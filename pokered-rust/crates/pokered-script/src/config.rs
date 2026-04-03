use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapScriptConfig {
    #[serde(default)]
    pub map_scripts: Vec<String>,
    #[serde(default)]
    pub npcs: Vec<NpcBinding>,
    #[serde(default)]
    pub signs: Vec<SignBinding>,
    #[serde(default)]
    pub coord_events: Vec<CoordEventBinding>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NpcBinding {
    pub id: u8,
    pub talk: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignBinding {
    pub id: u8,
    pub talk: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordEventBinding {
    pub position: (u16, u16),
    pub trigger: String,
}

impl MapScriptConfig {
    pub fn map_script_fn_name(&self, index: usize) -> Option<&str> {
        self.map_scripts.get(index).map(|s| s.as_str())
    }

    pub fn resolve_map_script_index(&self, state_name: &str) -> Option<usize> {
        self.map_scripts.iter().position(|s| s == state_name)
    }

    pub fn npc_talk_fn(&self, npc_text_id: u8) -> Option<&str> {
        self.npcs
            .iter()
            .find(|n| n.id == npc_text_id)
            .map(|n| n.talk.as_str())
    }

    pub fn sign_talk_fn(&self, sign_text_id: u8) -> Option<&str> {
        self.signs
            .iter()
            .find(|s| s.id == sign_text_id)
            .map(|s| s.talk.as_str())
    }

    pub fn coord_event_fn(&self, x: u16, y: u16) -> Option<&str> {
        self.coord_events
            .iter()
            .find(|c| c.position == (x, y))
            .map(|c| c.trigger.as_str())
    }
}
