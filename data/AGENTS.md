# data/ — Static Game Data Tables

5 loose files + 17 subdirectories. Pure data — no game logic. Referenced by engine/ code via labels.

## Subdirectory Map
| Subdirectory | Purpose |
|---|---|
| `pokemon/` | Base stats (per-species .asm), dex entries/text, cries, evolution/learnset data, menu icons |
| `moves/` | Move definitions (power, type, accuracy, PP, effect), animation pointers |
| `trainers/` | Trainer party definitions, AI move-choice tables, trainer names |
| `maps/` | Map headers, object data, connection data, tilemap blocks. Subdirs: `headers/`, `objects/` |
| `items/` | Item names, prices, TM/HM move mappings |
| `wild/` | Wild encounter tables per route/cave (species + level ranges) |
| `types/` | Type names, type effectiveness matchup table |
| `events/` | Event displacement tables, sign/hidden event data |
| `battle/` | Special battle effects, critical hit rates, badge stat boosts |
| `battle_anims/` | Animation framesets, subanimation pointers, tile patterns |
| `tilesets/` | Tileset collision data, tile animation definitions |
| `sprites/` | Sprite facing frame tables, OAM data |
| `text/` | Miscellaneous game text (not map dialog — that's in top-level text/) |
| `player/` | Player name defaults, starter choices |
| `credits/` | Credits sequence text and pointer tables |
| `sgb/` | Super Game Boy border/palette data |

## Loose Files (data root)
- `predef_pointers.asm` — function pointer table for `predef` dispatch
- `text_predef_pointers.asm` — text pointer table (many XXX unused entries documented)
- `icon_pointers.asm` — party menu icon pointers
- `growth_rates.asm` — EXP growth rate curves
- `text_boxes.asm` — text box dimension/layout definitions
- `tilemaps.asm` — misc tilemap data
- `yes_no_menu_strings.asm` — yes/no prompt strings

## Data Format Conventions
- Pokemon base stats: one .asm per species in `pokemon/base_stats/`, uses `db`/`dw` with named constants from `constants/pokemon_data_constants.asm` + `tmhm` macro for TM/HM learnability bitmask
- Move data: `moves/moves.asm` — table of `move` macro entries (effect, power, type, accuracy, PP)
- Map headers: `maps/headers/MapName.asm` — `map_header` macro with dimensions, tileset, connections
- Wild encounters: `wild/MapName.asm` — encounter rate + species/level pairs
- Trainer parties: `trainers/parties.asm` — trainer class + level/species lists
- All tables use `assert_table_length` / `assert_list_length` macros to validate sizes at assembly time
