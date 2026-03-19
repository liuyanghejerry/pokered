# macros/ — Assembly Macro System

9 general macros + `scripts/` subdirectory (4 domain-specific macro files). Defines project conventions and code generation patterns.

## Core Macros
- `const.asm` — `const_def`/`const`/`const_next`/`const_skip`/`shift_const`/`const_export`: the enumeration system used by ALL constant files. `const_value` is global state; `const_def` resets it
- `farcall.asm` — `farcall`/`callfar`/`farjp`/`jpfar` (cross-bank calls via Bankswitch), `homecall`/`homecall_sf` (call HOME preserving caller's bank)
- `asserts.asm` — `table_width`/`assert_table_length`, `list_start`/`li`/`assert_list_length`, `nybble_list`/`bit_array` — compile-time size validation. Build fails on mismatch
- `data.asm` — `dn` (nybble pairs), `dbw`/`dwb` (mixed db/dw), `dname` (padded names), `tmhm` (TM/HM learnability bitmask), `bcd2`/`bcd3` (BCD encoding)
- `predef.asm` — `predef`/`predef_jump`/`tx_pre`/`tx_pre_jump`/`db_tx_pre`: predefined function/text dispatch
- `code.asm` — syntactic sugar: `lb` (load byte pair into register pair), `ldpal` (palette register helper), `dict` (text dictionary entries)
- `coords.asm` — `coord`/`bgcoord`/`owcoord` variants for hl/bc/de tilemap addressing, `event_displacement`, `dbmapcoord`
- `gfx.asm` — `RGB` (palette color), `dbsprite` (sprite entry)
- `vc.asm` — `vc_hook`/`vc_patch`/`vc_assert`: Virtual Console patch macros (conditional on _RED_VC/_BLUE_VC)
- `ram.asm` — `box_struct`/`party_struct`/`battle_struct`/`sprite_oam_struct`/`map_connection_struct`/`flag_array`: RAM structure definitions

## scripts/ — Domain Scripting Macros
- `scripts/text.asm` — `text`/`next`/`done`/`prompt`, `text_ram`/`text_decimal`/`text_bcd`, `script_pokecenter_nurse`/`script_mart`, TX_* command wrappers
- `scripts/events.asm` — `CheckEvent`/`SetEvent`/`ResetEvent`/`CheckAndSetEvent` + reuse variants (ReuseA/ReuseHL) for bit-packed wEventFlags
- `scripts/maps.asm` — `map_header`/`end_map_header`, `connection`, `object_event`/`warp_event`/`trainer`, `def_object_events`/`def_warps_to` with counter assertions
- `scripts/audio.asm` — `channel_count`/`channel`, `note`/`rest`/`speed`/`tempo`, `vibrato`/`pitch_slide`/`duty_cycle`, `execute_music`

## Include Order (via includes.asm)
1. General macros: asserts → const → predef → farcall → data → code → gfx → coords → vc
2. Script macros: audio → maps → events → text
3. Constants (after macros so macros are available for constant definitions)

## Convention: const_def Cannot Go Backwards
`const_next` jumps forward only. `const_value` is shared global state within a const_def block. Misuse causes silent misnumbering.
