# scripts/ — Map Event Scripts

224 files, one per map. Each defines the event logic for a specific game location: trainer battles, item pickups, NPC interactions, warp triggers, and scripted sequences.

## File Pattern
Every file follows `scripts/MapName.asm` naming. Each map script is paired with:
- `text/MapName.asm` — dialog text strings for that map's NPCs
- `data/maps/headers/MapName.asm` — map header (dimensions, tileset, connections)
- `data/maps/objects/MapName.asm` — object/warp/sign definitions

## Common Structure
```asm
MapName_Script:           ; main script entry (called on map load)
    ; check/set event flags, run scripted movement sequences
MapName_TextPointers:     ; text ID → text handler table
MapNameText1:             ; individual NPC text handlers
    ; may call EngageTrainer, GiveItem, CheckEvent, etc.
```

## Key Macros Used
- `CheckEvent` / `SetEvent` / `ResetEvent` — test/set/clear event flags in wEventFlags
- `EngineFlags*` — trainer engagement and battle triggers
- Text macros from `macros/scripts/text.asm`: `text`, `done`, `prompt`
- Object/warp setup via `macros/scripts/maps.asm`

## Notable Scripts
- `OaksLab.asm` — starter selection sequence, rival battle trigger, parcel quest
- `PalletTown.asm` — game opening, Oak encounter scripted walk
- `SilphCo7F.asm` — Giovanni battle + Lapras gift
- `PokemonTower7F.asm` — Marowak ghost battle (special ghost mechanics)
- `CeruleanCity.asm` / `CeruleanCity_2.asm` — rival battle on Nugget Bridge

## Known Bugs (preserved for ROM accuracy)
- `PewterCity.asm` — BUG: loads $40 instead of $50, causing sprite misalignment
- `RocketHideoutB1F.asm` — BUG: should use SetEvent for SFX; replays every map entry instead

## Convention
Scripts are purely event-driven. Game logic lives in `engine/`. Scripts compose engine calls with event flag checks to create the game's storyline progression.
