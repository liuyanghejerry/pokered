# home/ — ROM0 Bank (Always-Mapped Routines)

64 files. Bank 0 is permanently mapped — every routine here is callable from any ROM bank without switching.

## Startup & Interrupt Flow
- `header.asm` — interrupt vectors: VBlank→`jp VBlank`, Timer→`jp Timer`, Serial→`jp Serial`; ROM header with `Start::` → `jp _Start`
- `start.asm` — `_Start` label; checks CGB flag, jumps to Init
- `init.asm` — `Init`/`SoftReset`: clears WRAM/HRAM/VRAM, sets stack, enables interrupts (VBlank/Timer/Serial), writes DMA code to HRAM, calls `PrepareTitleScreen`
- `vblank.asm` — VBlank handler: runs AutoBgMapTransfer, VBlankCopy, PrepareOAMData, music tick, ReadJoypad, random seed update. Central frame-tick entry point

## Bank Switching
- `bankswitch.asm` — `Bankswitch` (switch to bank in B + call HL), `BankswitchHome`, `BankswitchBack`. Reads/writes `hLoadedROMBank` (HRAM) and `rROMB` (HW register)
- Cross-bank call macros live in `macros/farcall.asm` — `farcall`/`callfar` set B=BANK, HL=label, call Bankswitch; `homecall` preserves caller's bank

## Core Subsystems
- **Text engine**: `text.asm` (PlaceString, text command processor), `text_script.asm` (DisplayTextID dispatch), `print_text.asm` (PrintLetterDelay), `textbox.asm` (draw text boxes)
- **Memory copy**: `copy.asm` (CopyData, FarCopyData), `copy2.asm` (FarCopyData2/3, CopyVideoData), `copy_string.asm`
- **VRAM transfer**: `vcopy.asm` (GetRowColAddressBgMap, ClearBgMap, RedrawRowOrColumn, AutoBgMapTransfer, VBlankCopy/Double, UpdateMovingBgTiles)
- **Input**: `joypad.asm` (ReadJoypad, main polling), `joypad2.asm` (secondary helpers)
- **LCD control**: `lcd.asm` (DisableLCD/EnableLCD)
- **OAM/sprites**: `oam.asm` (WriteOAMBlock), `clear_sprites.asm`, `update_sprites.asm`, `reload_sprites.asm`, `reset_player_sprite.asm`
- **Overworld**: `overworld.asm` (player movement, map loading — contains XXX uncertain-reachability notes)
- **Audio dispatch**: `audio.asm` (home→audio bank switching), `fade_audio.asm`

## Utility Functions (used project-wide)
- `math.asm` — integer multiply/divide
- `random.asm` — RNG
- `delay.asm` — DelayFrame/DelayFrames/Delay3
- `compare.asm` — memcmp-like
- `count_set_bits.asm` — popcount utility
- `print_num.asm` / `print_bcd.asm` — numeric formatting
- `palettes.asm` — GB palette register helpers
- `load_font.asm` — load text font into VRAM
- `serial.asm` — link cable serial interrupt + exchange
- `predef.asm` / `predef_text.asm` — predefined text lookup

## Gotchas
- `overworld.asm` has "XXX can this code be reached?" comments — uncertain control flow paths preserved for byte-accuracy
- `trainers.asm` has multiple XXX notes on suspicious logic
- `names2.asm` contains BUG references in name handling
- All HOME routines are in bank 0 — adding code here that exceeds 16KB will break the build
