//! Per-map dialog text data for NPCs and signs.
//!
//! Each map has two text tables:
//! - NPC texts: indexed by the NPC's text_id (1-based, matching the map's TextPointers table)
//! - Sign texts: indexed by the sign's text_id (1-based, separate namespace from NPCs)
//!
//! Each text entry is a slice of string slices, where each string is one "page" of dialog
//! (displayed two lines at a time in the text box, advanced with A-button).
//!
//! For context-dependent NPCs (e.g., Oak changes dialog based on game progress),
//! we return the default/earliest dialog variant.
//!
//! Text uses the original game's formatting conventions:
//! - `<PLAYER>` and `<RIVAL>` are placeholders resolved at runtime
//! - `#MON` is the abbreviation for Pokémon
//! - `#DEX` is the abbreviation for Pokédex

use crate::maps::MapId;

/// A single dialog page displayed in the text box (2 lines).
#[derive(Debug, Clone, Copy)]
pub struct TextPage {
    pub line1: &'static str,
    pub line2: &'static str,
}

/// A complete dialog sequence (one or more pages).
pub type DialogText = &'static [TextPage];

/// Empty dialog (no text to show).
const EMPTY_DIALOG: &[TextPage] = &[];

// ── Pallet Town NPC texts ──────────────────────────────────────────
// TextPointers entries 1-3 map to NPC text_ids 1-3.
// Entry 1: Oak (default = "It's unsafe" variant)
// Entry 2: Girl
// Entry 3: Fisher

const PALLETTOWN_NPC_OAK: &[TextPage] = &[
    TextPage {
        line1: "OAK: It's unsafe!",
        line2: "Wild POKeMON live",
    },
    TextPage {
        line1: "in tall grass!",
        line2: "",
    },
    TextPage {
        line1: "You need your own",
        line2: "POKeMON for your",
    },
    TextPage {
        line1: "protection.",
        line2: "I know!",
    },
    TextPage {
        line1: "Here, come with",
        line2: "me!",
    },
];

const PALLETTOWN_NPC_GIRL: &[TextPage] = &[
    TextPage {
        line1: "I'm raising",
        line2: "POKeMON too!",
    },
    TextPage {
        line1: "When they get",
        line2: "strong, they can",
    },
    TextPage {
        line1: "protect me!",
        line2: "",
    },
];

const PALLETTOWN_NPC_FISHER: &[TextPage] = &[
    TextPage {
        line1: "Technology is",
        line2: "incredible!",
    },
    TextPage {
        line1: "You can now store",
        line2: "and recall items",
    },
    TextPage {
        line1: "and POKeMON as",
        line2: "data via PC!",
    },
];

/// PalletTown NPC texts. Index = text_id - 1.
const PALLETTOWN_NPC_TEXTS: &[DialogText] = &[
    PALLETTOWN_NPC_OAK,    // text_id 1
    PALLETTOWN_NPC_GIRL,   // text_id 2
    PALLETTOWN_NPC_FISHER, // text_id 3
];

// ── Pallet Town Sign texts ─────────────────────────────────────────
// Signs have their own text_id namespace (1-based).
// sign text_id 1 → OAK POKeMON RESEARCH LAB
// sign text_id 2 → PALLET TOWN / Shades of your journey await!
// sign text_id 3 → <PLAYER>'s house
// sign text_id 4 → <RIVAL>'s house

const PALLETTOWN_SIGN_OAKSLAB: &[TextPage] = &[TextPage {
    line1: "OAK POKeMON",
    line2: "RESEARCH LAB",
}];

const PALLETTOWN_SIGN_TOWN: &[TextPage] = &[
    TextPage {
        line1: "PALLET TOWN",
        line2: "Shades of your",
    },
    TextPage {
        line1: "journey await!",
        line2: "",
    },
];

const PALLETTOWN_SIGN_PLAYER_HOUSE: &[TextPage] = &[TextPage {
    line1: "<PLAYER>'s house",
    line2: "",
}];

const PALLETTOWN_SIGN_RIVAL_HOUSE: &[TextPage] = &[TextPage {
    line1: "<RIVAL>'s house",
    line2: "",
}];

/// PalletTown sign texts. Index = text_id - 1.
const PALLETTOWN_SIGN_TEXTS: &[DialogText] = &[
    PALLETTOWN_SIGN_OAKSLAB,      // sign text_id 1
    PALLETTOWN_SIGN_TOWN,         // sign text_id 2
    PALLETTOWN_SIGN_PLAYER_HOUSE, // sign text_id 3
    PALLETTOWN_SIGN_RIVAL_HOUSE,  // sign text_id 4
];

// ── Red's House 1F NPC texts ──────────────────────────────────────
// TextPointers entry 1: Mom (default = wake-up text before getting starter)
// TextPointers entry 2: TV (handled specially in original — facing-dependent)

const REDSHOUSE1F_NPC_MOM: &[TextPage] = &[
    TextPage {
        line1: "MOM: Right.",
        line2: "All boys leave",
    },
    TextPage {
        line1: "home some day.",
        line2: "It said so on TV.",
    },
    TextPage {
        line1: "PROF.OAK, next",
        line2: "door, is looking",
    },
    TextPage {
        line1: "for you.",
        line2: "",
    },
];

/// TV text (Stand By Me movie) — shown when facing up at the TV.
const REDSHOUSE1F_NPC_TV: &[TextPage] = &[
    TextPage {
        line1: "There's a movie",
        line2: "on TV. Four boys",
    },
    TextPage {
        line1: "are walking on",
        line2: "railroad tracks.",
    },
    TextPage {
        line1: "I better go too.",
        line2: "",
    },
];

/// RedsHouse1F NPC texts. Index = text_id - 1.
const REDSHOUSE1F_NPC_TEXTS: &[DialogText] = &[
    REDSHOUSE1F_NPC_MOM, // text_id 1
    REDSHOUSE1F_NPC_TV,  // text_id 2
];

// ── Red's House 1F Sign texts ──────────────────────────────────────
// RedsHouse1F has no signs in the objects file — the TV is an NPC, not a sign.
const REDSHOUSE1F_SIGN_TEXTS: &[DialogText] = &[];

// ── Red's House 2F ────────────────────────────────────────────────
// No NPCs, no signs (SNES interaction is handled separately by script_engine).
const REDSHOUSE2F_NPC_TEXTS: &[DialogText] = &[];
const REDSHOUSE2F_SIGN_TEXTS: &[DialogText] = &[];

// ── Blue's House NPC texts ─────────────────────────────────────────
// TextPointers entry 1: Daisy sitting (default = rival at lab)
// TextPointers entry 2: Daisy walking
// TextPointers entry 3: Town Map object

const BLUESHOUSE_NPC_DAISY_SITTING: &[TextPage] = &[
    TextPage {
        line1: "Hi <PLAYER>!",
        line2: "<RIVAL> is out at",
    },
    TextPage {
        line1: "Grandpa's lab.",
        line2: "",
    },
];

const BLUESHOUSE_NPC_DAISY_WALKING: &[TextPage] = &[
    TextPage {
        line1: "POKeMON are living",
        line2: "things! If they",
    },
    TextPage {
        line1: "get tired, give",
        line2: "them a rest!",
    },
];

const BLUESHOUSE_NPC_TOWN_MAP: &[TextPage] = &[TextPage {
    line1: "It's a big map!",
    line2: "This is useful!",
}];

/// BluesHouse NPC texts. Index = text_id - 1.
const BLUESHOUSE_NPC_TEXTS: &[DialogText] = &[
    BLUESHOUSE_NPC_DAISY_SITTING, // text_id 1
    BLUESHOUSE_NPC_DAISY_WALKING, // text_id 2
    BLUESHOUSE_NPC_TOWN_MAP,      // text_id 3
];

const BLUESHOUSE_SIGN_TEXTS: &[DialogText] = &[];

// ── Oak's Lab NPC texts ────────────────────────────────────────────
// TextPointers entries 1-11 map to NPC text_ids 1-11.
// Many are context-dependent; we use the default/first variant.

const OAKSLAB_NPC_RIVAL: &[TextPage] = &[
    TextPage {
        line1: "<RIVAL>: Yo",
        line2: "<PLAYER>! Gramps",
    },
    TextPage {
        line1: "isn't around!",
        line2: "",
    },
];

const OAKSLAB_NPC_POKEBALL_CHARMANDER: &[TextPage] = &[
    TextPage {
        line1: "Those are POKe",
        line2: "BALLs. They",
    },
    TextPage {
        line1: "contain POKeMON!",
        line2: "",
    },
];

const OAKSLAB_NPC_POKEBALL_SQUIRTLE: &[TextPage] = &[
    TextPage {
        line1: "Those are POKe",
        line2: "BALLs. They",
    },
    TextPage {
        line1: "contain POKeMON!",
        line2: "",
    },
];

const OAKSLAB_NPC_POKEBALL_BULBASAUR: &[TextPage] = &[
    TextPage {
        line1: "Those are POKe",
        line2: "BALLs. They",
    },
    TextPage {
        line1: "contain POKeMON!",
        line2: "",
    },
];

const OAKSLAB_NPC_OAK1: &[TextPage] = &[
    TextPage {
        line1: "OAK: If a wild",
        line2: "POKeMON appears,",
    },
    TextPage {
        line1: "your POKeMON can",
        line2: "fight against it!",
    },
];

const OAKSLAB_NPC_POKEDEX1: &[TextPage] = &[
    TextPage {
        line1: "It's encyclopedia-",
        line2: "like, but the",
    },
    TextPage {
        line1: "pages are blank!",
        line2: "",
    },
];

const OAKSLAB_NPC_POKEDEX2: &[TextPage] = &[
    TextPage {
        line1: "It's encyclopedia-",
        line2: "like, but the",
    },
    TextPage {
        line1: "pages are blank!",
        line2: "",
    },
];

const OAKSLAB_NPC_OAK2: &[TextPage] = &[TextPage {
    line1: "?",
    line2: "",
}];

const OAKSLAB_NPC_GIRL: &[TextPage] = &[
    TextPage {
        line1: "PROF.OAK is the",
        line2: "authority on",
    },
    TextPage {
        line1: "POKeMON!",
        line2: "",
    },
    TextPage {
        line1: "Many POKeMON",
        line2: "trainers hold him",
    },
    TextPage {
        line1: "in high regard!",
        line2: "",
    },
];

const OAKSLAB_NPC_SCIENTIST1: &[TextPage] = &[TextPage {
    line1: "I study POKeMON as",
    line2: "PROF.OAK's AIDE.",
}];

const OAKSLAB_NPC_SCIENTIST2: &[TextPage] = &[TextPage {
    line1: "I study POKeMON as",
    line2: "PROF.OAK's AIDE.",
}];

/// OaksLab NPC texts. Index = text_id - 1.
const OAKSLAB_NPC_TEXTS: &[DialogText] = &[
    OAKSLAB_NPC_RIVAL,               // text_id 1
    OAKSLAB_NPC_POKEBALL_CHARMANDER, // text_id 2
    OAKSLAB_NPC_POKEBALL_SQUIRTLE,   // text_id 3
    OAKSLAB_NPC_POKEBALL_BULBASAUR,  // text_id 4
    OAKSLAB_NPC_OAK1,                // text_id 5
    OAKSLAB_NPC_POKEDEX1,            // text_id 6
    OAKSLAB_NPC_POKEDEX2,            // text_id 7
    OAKSLAB_NPC_OAK2,                // text_id 8
    OAKSLAB_NPC_GIRL,                // text_id 9
    OAKSLAB_NPC_SCIENTIST1,          // text_id 10
    OAKSLAB_NPC_SCIENTIST2,          // text_id 11
];

const OAKSLAB_SIGN_TEXTS: &[DialogText] = &[];

// ── Public API ──────────────────────────────────────────────────────

/// Get NPC dialog text for a given map and NPC text_id.
/// Returns a slice of TextPages for displaying in the dialog box.
/// text_id is 1-based (matches the original game's TextPointers table).
pub fn get_npc_text(map: MapId, text_id: u8) -> DialogText {
    let texts = match map {
        MapId::PalletTown => PALLETTOWN_NPC_TEXTS,
        MapId::RedsHouse1F => REDSHOUSE1F_NPC_TEXTS,
        MapId::RedsHouse2F => REDSHOUSE2F_NPC_TEXTS,
        MapId::BluesHouse => BLUESHOUSE_NPC_TEXTS,
        MapId::OaksLab => OAKSLAB_NPC_TEXTS,
        _ => return EMPTY_DIALOG,
    };

    let idx = (text_id as usize).wrapping_sub(1);
    texts.get(idx).copied().unwrap_or(EMPTY_DIALOG)
}

/// Get sign dialog text for a given map and sign text_id.
/// Returns a slice of TextPages for displaying in the dialog box.
/// text_id is 1-based (signs have their own namespace separate from NPCs).
pub fn get_sign_text(map: MapId, text_id: u8) -> DialogText {
    let texts = match map {
        MapId::PalletTown => PALLETTOWN_SIGN_TEXTS,
        MapId::RedsHouse1F => REDSHOUSE1F_SIGN_TEXTS,
        MapId::RedsHouse2F => REDSHOUSE2F_SIGN_TEXTS,
        MapId::BluesHouse => BLUESHOUSE_SIGN_TEXTS,
        MapId::OaksLab => OAKSLAB_SIGN_TEXTS,
        _ => return EMPTY_DIALOG,
    };

    let idx = (text_id as usize).wrapping_sub(1);
    texts.get(idx).copied().unwrap_or(EMPTY_DIALOG)
}
