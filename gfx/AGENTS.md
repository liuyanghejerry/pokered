# gfx/ — Graphics Assets + Loading Code

7 loader .asm files + 18 subdirectories of assets. Handles the full visual pipeline from PNG source → compressed Game Boy tiles.

## Loader Files (gfx root)
- `sprites.asm` — overworld sprite tileset loading
- `pics.asm` — Pokémon pic (front/back sprite) loading and decompression dispatch
- `tilesets.asm` — tileset loading for map rendering
- `font.asm` — font tile loading
- `player.asm` — player sprite graphics loading
- `fishing.asm` — fishing minigame graphics
- `trade.asm` — trade sequence graphics
- `trainer_card.asm` — trainer card UI graphics
- `version.asm` — version-specific (Red/Blue) graphics loading

## Asset Subdirectories
| Subdirectory | Content |
|---|---|
| `pokemon/` | Front/back sprites per species (.png source + .pic compressed + .2bpp tiles) |
| `trainers/` | Trainer class portrait sprites |
| `sprites/` | Overworld character sprites (walking frames) |
| `tilesets/` | Map tileset graphics (.2bpp) |
| `blocksets/` | Map block definitions (.bst files — 4×4 tile blocks) |
| `overworld/` | Overworld UI elements (player, items, cut tree, etc.) |
| `font/` | Text font tiles, badge letters |
| `icons/` | Party menu mon type icons |
| `title/` | Title screen graphics (Red/Blue variants) |
| `intro/` | Intro sequence frames (Nidorino/Gengar fight) |
| `splash/` | Game Freak splash screen |
| `battle/` | Battle UI elements (HP bar, status icons) |
| `pokedex/` | Pokédex screen graphics |
| `town_map/` | Town map tileset |
| `trade/` | Trade animation graphics |
| `trainer_card/` | Trainer card elements |
| `credits/` | Credits sequence graphics |
| `sgb/` | Super Game Boy border graphics |
| `slots/` | Slot machine graphics |
| `player/` | Player sprites (cycling, surfing, fishing frames) |

## Graphics Pipeline
1. Source PNGs in asset subdirectories
2. `rgbgfx` converts PNG → `.2bpp` (2 bits per pixel) or `.1bpp` (1bpp for fonts)
3. `tools/gfx` applies tiling/interleaving transformations
4. `tools/pkmncompress` compresses Pokémon sprites → `.pic` format
5. Makefile rules automate: `%.2bpp: %.png` and `%.pic: %.2bpp`
6. Compressed data is INCBINed into ROM via loader .asm files
