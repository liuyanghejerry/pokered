# constants/ — Shared Constant Definitions

37 files. All ID enumerations, hardware registers, flag masks, and struct offsets. Included project-wide via `includes.asm`.

## Most-Referenced Files (start here)
- `pokemon_constants.asm` — species IDs (NO_MON, RHYDON..MEW). Canonical species enumeration
- `item_constants.asm` — item IDs + badge constants + `add_tm`/`add_hm` macros for TM/HM numbering
- `move_constants.asm` — move IDs (NO_MOVE, POUND..STRUGGLE), NUM_ATTACKS
- `map_constants.asm` — `map_const` macro + full map ID list with width/height per map
- `event_constants.asm` — wEventFlags bit indices (very large — enumerates every game event flag)
- `charmap.asm` — text encoding charmap (NOT ASCII). Control codes: `<LINE>`, `<PAGE>`, `<DONE>`, `<PLAYER>`, `<RIVAL>`, `<PKMN>`

## Domain-Specific Constants
- `battle_constants.asm` — NUM_MOVES, MAX_LEVEL, stat indices, status flags, damage scaling
- `move_effect_constants.asm` — EFFECT_* IDs for move behaviors
- `trainer_constants.asm` — trainer class IDs via `trainer_const` macro
- `music_constants.asm` — music/SFX ID enumeration via `music_const` macro
- `audio_constants.asm` — hardware channel masks, audio flag bits, note/chan constants

## Structure & Layout Constants
- `pokemon_data_constants.asm` — base stat struct offsets (BASE_HP, MON_HP, MON_MOVES), party/box sizes
- `map_data_constants.asm` — map layout constants, connection bits, MAX_WARP_EVENTS
- `map_object_constants.asm` — sprite state struct offsets (SPRITESTATEDATA1_*, SPRITESTATEDATA2_*)
- `gfx_constants.asm` — tile/screen/sprite sizes, tilemap IDs
- `ram_constants.asm` — runtime flag definitions (wStatusFlags, badge bits, options masks)
- `text_constants.asm` — name lengths, PrintNumber flags

## Platform & Misc
- `hardware.inc` — Game Boy hardware registers (rJOYP, rLCDC, rIE, APU regs, etc.)
- `sprite_constants.asm` — overworld sprite IDs
- `palette_constants.asm` — SGB palette packet/set IDs
- `type_constants.asm` — type IDs (physical/special groups)
- `serial_constants.asm` — link protocol constants
- `misc_constants.asm` — TRUE/FALSE, JOYP bits

## Enumeration Pattern
All files use `const_def` + `const` macros from `macros/const.asm`. Never hardcode numeric IDs — always add entries via `const` to maintain auto-incrementing. Use `const_skip` to reserve gaps, `shift_const` for bitmask enumerations.
