use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapScriptConfig {
    #[serde(default)]
    pub on_load: Option<String>,
    #[serde(default)]
    pub npcs: Vec<NpcBinding>,
    #[serde(default)]
    pub signs: Vec<SignBinding>,
    #[serde(default)]
    pub coord_events: Vec<CoordEventBinding>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcBinding {
    pub id: u8,
    #[serde(default)]
    pub talk: Option<String>,
    /// Named toggle identifier for script showObject/hideObject (e.g. "PALLET_TOWN_OAK").
    #[serde(default)]
    pub toggle_id: Option<String>,
    /// Script-facing NPC identifier used by moveNpc/startNpcMove (e.g. "PALLETTOWN_OAK").
    #[serde(default)]
    pub script_id: Option<String>,
    /// If true, this NPC is hidden when the map first loads (until a script shows it).
    #[serde(default)]
    pub default_hidden: bool,
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
    pub fn on_load(&self) -> Option<&str> {
        self.on_load.as_deref()
    }

    pub fn npc_talk_fn(&self, npc_text_id: u8) -> Option<&str> {
        self.npcs
            .iter()
            .find(|n| n.id == npc_text_id)
            .and_then(|n| n.talk.as_deref())
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

    pub fn hidden_npc_ids(&self) -> Vec<u8> {
        self.npcs
            .iter()
            .filter(|n| n.default_hidden)
            .map(|n| n.id)
            .collect()
    }

    pub fn npc_id_by_toggle(&self, toggle_id: &str) -> Option<u8> {
        self.npcs
            .iter()
            .find(|n| n.toggle_id.as_deref() == Some(toggle_id))
            .map(|n| n.id)
    }

    pub fn npc_id_by_script_id(&self, script_id: &str) -> Option<u8> {
        self.npcs
            .iter()
            .find(|n| n.script_id.as_deref() == Some(script_id))
            .map(|n| n.id)
    }
}
