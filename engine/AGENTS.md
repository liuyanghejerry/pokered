# engine/ — Game Logic (ROMX Banks)

14 subsystems across switchable ROM banks. All engine code runs via `farcall`/`callfar` from HOME or other banks.

## Subsystem Map
| Subdirectory | Files | Purpose |
|---|---|---|
| `battle/` | 25 + move_effects/ | Battle system — the largest and most complex subsystem |
| `overworld/` | ~15 | Player movement, map transitions, sprite collisions, trainer sight |
| `menus/` | ~15 | Start menu, Pokédex, naming screen, save, list menus, text boxes |
| `events/` | ~12 | Pokemart, hidden items/events, prize menu, black out, vending machine |
| `items/` | ~5 | Item effects, TM/HM usage, item-use flows |
| `pokemon/` | ~8 | Evolution/moves, party management, Pokédex data, mon removal |
| `gfx/` | ~8 | Sprite OAM, palettes, mon icons, HP bar, screen effects |
| `link/` | ~3 | Cable club, link trade/battle protocol |
| `movie/` | ~5 | Intro sequence, Oak speech, title screen, splash, hall of fame |
| `slots/` | ~2 | Slot machine logic |
| `math/` | ~2 | Extended math (multiply, divide beyond HOME helpers) |
| `debug/` | ~2 | Debug party setup (only in debug builds) |

## Loose Files (engine root)
- `predefs.asm` — predefined function pointer table (jumped to via `predef` macro)
- `flag_action.asm` — bit flag get/set/reset operations on arbitrary memory
- `joypad.asm` — extended joypad handling (beyond home/joypad.asm)
- `play_time.asm` — play time counter increment

## Cross-Subsystem Patterns
- Engine code calls HOME routines directly (they're always mapped)
- Engine→engine calls across banks use `farcall` (sets bank B + address HL, calls Bankswitch)
- `predefs.asm` is the dispatch table for indirect calls — `predef` macro looks up function pointer by index
- Battle system is the heaviest consumer of cross-bank calls (battle→gfx, battle→pokemon, battle→audio)

## Key Architectural Notes
- Each subsystem's code is placed into specific ROM banks via `main.asm` SECTION declarations + `layout.link`
- Many files contain `BUG:` / `XXX` comments preserving original game bugs for ROM matching
- `engine/events/hidden_items.asm` has "due to a typo, this is never used" (documented dead code)
- `engine/battle/unused_stats_functions.asm` — preserved unused functions for byte-accuracy
