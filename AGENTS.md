# pokered — Pokémon Red/Blue RGBDS Disassembly

## What This Is
Complete disassembly of Pokémon Red and Blue (Game Boy). Builds byte-identical ROMs verified against canonical SHA1 sums. Community project by pret.

## Architecture

### Hardware Target
- Game Boy LR35902 CPU (Z80-like), 1MHz
- 1MB ROM: 32 banks × 16KB. Bank 0 (ROM0) always mapped; banks 1-31 switchable via MBC3
- 8KB WRAM, 8KB VRAM, 160B OAM, 127B HRAM

### ROM Layout
- `layout.link` — assigns code/data sections to ROM banks
- `main.asm` — master ROMX section manifest (INCLUDE all engine/data/script/text/gfx/audio files into numbered sections)
- `home.asm` — ROM0 section manifest (always-accessible routines)
- `includes.asm` — global include: macros first, then constants, then charmap. Every .asm file INCLUDEs this

### Bank Switching
- `home/bankswitch.asm` — Bankswitch/BankswitchHome/BankswitchBack implementations
- `macros/farcall.asm` — `farcall`/`callfar` (call code in another bank), `homecall` (call HOME from any bank)
- hLoadedROMBank (HRAM) tracks current bank; rROMB is the HW register

### Red vs Blue
Conditional compilation: `-D _RED` or `-D _BLUE` in Makefile. Affects starters, version-exclusive Pokémon, title screen, sprite palettes. Search `IF DEF(_RED)` / `IF DEF(_BLUE)` for differences.

## Build System
- **Toolchain**: rgbds v1.0.1 (rgbasm/rgblink/rgbfix/rgbgfx) + custom C tools in `tools/`
- **Build**: `make` (both ROMs), `make red`, `make blue`, `make compare` (build + SHA1 verify)
- **CI**: GitHub Actions (.github/workflows/main.yml) — ubuntu + macOS, builds + SHA1 compare + checkdiff.sh
- **Verification**: `make compare` runs `sha1sum -c roms.sha1`; assembly-time ASSERTs in `macros/asserts.asm` catch table size mismatches

### Graphics Pipeline
PNG → `rgbgfx` → .2bpp/.1bpp → `tools/gfx` (tiling/interleaving) → .pic via `tools/pkmncompress` (Pokémon sprite compression)

## Directory Map
| Directory | Purpose | Details |
|-----------|---------|---------|
| `home/` | ROM0 bank — always-mapped routines | VBlank, bankswitch, text engine, copy, input, init (64 files) |
| `engine/` | Game logic across ROMX banks | 14 subsystems: battle, overworld, menus, items, pokemon, gfx, events, link, movie, slots, math, debug |
| `data/` | Static game data tables | Pokemon stats, moves, maps, trainers, items, wild encounters, types (17 subdirs) |
| `constants/` | Shared constant definitions | Pokemon/item/move/map/event IDs, hardware regs, charmap (37 files) |
| `macros/` | Assembly macros | farcall, const_def, text/map/event scripting, data packing, assertions (12 files) |
| `scripts/` | Map event scripts | One file per map — trainer battles, item pickups, NPC dialog triggers (224 files) |
| `text/` | Dialog text | One file per map — NPC dialog strings using charmap encoding (211 files) |
| `audio/` | Sound engine + music/SFX data | 3 duplicate engines (banks 2/8/$1F), ~30 BGM, ~100 SFX |
| `gfx/` | Graphics assets + loading code | Pokemon sprites, tilesets, overworld, UI elements (18 subdirs) |
| `ram/` | RAM layout | wram.asm (work RAM), hram.asm (high RAM), sram.asm (save RAM) |
| `tools/` | Build helper programs (C) | gfx.c (tile ops), pkmncompress.c (sprite compress), make_patch.c (VC patches), scan_includes.c |
| `vc/` | Virtual Console patches | Patch templates + constants for 3DS VC releases |

## Key Conventions
- **const_def/const pattern**: All ID enumerations use `const_def` + `const` macros (never hardcode numeric IDs)
- **Event flags**: Bit-packed in wEventFlags; use `CheckEvent`/`SetEvent`/`ResetEvent` macros from `macros/scripts/events.asm`
- **Map scripts**: Each map has `scripts/MapName.asm` (events/logic) + `text/MapName.asm` (dialog) + `data/maps/headers/MapName.asm` (header)
- **Labels**: `PascalCase` for routines, `ALL_CAPS` for constants, `wPrefix` for WRAM vars, `hPrefix` for HRAM
- **Text encoding**: Custom charmap in `constants/charmap.asm` — NOT ASCII. Control codes: `<LINE>`, `<PAGE>`, `<DONE>`, `<PLAYER>`, `<RIVAL>`

## Known Gotchas
- Many files have `BUG:` / `XXX` comments marking original game bugs preserved for byte-accuracy
- `scripts/PewterCity.asm` — sprite misalignment bug ($40 instead of $50)
- `scripts/RocketHideoutB1F.asm` — SFX replays on every map entry (should be SetEvent)
- Audio engines are triplicated (engine_1/2/3) — they are identical code in different banks
- `ram/wram.asm` has many "always 0" / "never read" fields — dead data preserved for ROM matching
- Modifying ANY byte changes SHA1 — use `make compare` to verify after changes

## Files at This Level
- `main.asm` — ROM section manifest (DO NOT reorder without updating layout.link)
- `home.asm` — ROM0 section manifest
- `includes.asm` — macro/constant include order (include this in any new .asm file)
- `layout.link` — ROM bank assignments (must match SECTION declarations)
- `ram.asm` / `audio.asm` / `maps.asm` / `text.asm` — aggregator includes for RAM/audio/maps/text
- `Makefile` — build rules, tool compilation, ROM targets
- `roms.sha1` — canonical checksums for built ROMs
- `INSTALL.md` — build setup instructions (rgbds v1.0.1 required; WSL/Cygwin path caveats)
