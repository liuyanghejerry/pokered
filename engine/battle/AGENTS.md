# engine/battle/ — Battle System

The most complex subsystem: 25 files + `move_effects/` subdirectory. `core.asm` alone is 6,913 lines.

## Core Files
- `core.asm` — Main battle loop, turn execution, damage calculation, move selection, switching, fainting, victory/loss. Contains many `XXX` comments about uncertain branches. THE central file for battle logic
- `effects.asm` — Move effect dispatch table and effect handler implementations (stat changes, status, multi-hit, etc.)
- `trainer_ai.asm` — AI decision-making: move selection heuristics, type awareness, item usage
- `animations.asm` — Battle animation system: subanimation tables, frameblock rendering, screen effects

## Battle Flow Files
- `init_battle_variables.asm` — Pre-battle setup (clear battle RAM, load species data)
- `read_trainer_party.asm` — Parse trainer party data from ROM (XXX comments about format uncertainty)
- `wild_encounters.asm` — Wild encounter checks and species/level selection
- `experience.asm` — EXP gain calculation and level-up flow
- `end_of_battle.asm` — Post-battle cleanup, reward money, trainer flag setting
- `safari_zone.asm` — Safari Zone-specific battle variant (rock/bait/ball)

## Display & UI
- `common_text.asm` — Battle message strings and text ID lookup
- `used_move_text.asm` — "[MON] used [MOVE]!" text generation
- `display_effectiveness.asm` — "It's super effective!" / "not very effective" logic
- `print_type.asm` — Type name display
- `draw_hud_pokeball_gfx.asm` — Party status pokeball indicators
- `link_battle_versus_text.asm` — VS screen for link battles
- `battle_transitions.asm` — Screen transition effects entering battle
- `scale_sprites.asm` / `scroll_draw_trainer_pic.asm` — Sprite scaling and trainer pic rendering
- `ghost_marowak_anim.asm` — Marowak ghost special animation

## Utility
- `decrement_pp.asm` — PP usage tracking
- `get_trainer_name.asm` / `save_trainer_name.asm` — Trainer name I/O
- `misc.asm` — Miscellaneous battle helpers (XXX notes about no-op functions)
- `unused_stats_functions.asm` — Dead code preserved for byte-accuracy

## move_effects/
Individual files per move effect (substitute.asm, transform.asm, etc.). Each implements a specific move's unique behavior. Contains BUG/NOTE comments about edge cases in substitute, transform, and others.

## Gotchas
- `core.asm` has "XXX does this ever jump?" markers — uncertain control flow preserved for ROM accuracy
- Many move effects have documented edge-case bugs (Gen 1 battle mechanics quirks)
- `read_trainer_party.asm` has format-uncertainty comments
- Battle↔audio, battle↔gfx cross-bank calls are frequent — trace via `farcall` usage
