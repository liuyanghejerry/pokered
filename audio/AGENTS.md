# audio/ — Sound Engine + Music/SFX Data

3 duplicate sound engines (banks 2, 8, $1F) + music compositions + SFX definitions. 4-channel Game Boy APU.

## Sound Engines
- `engine_1.asm` / `engine_2.asm` / `engine_3.asm` — **Identical code** in three ROM banks. The game switches between them depending on which bank is currently loaded. Contains BUG comments and "appears to never be used" annotations. Triplicated for bank-availability reasons (the engine must be callable from different contexts)

## Support Files
- `notes.asm` — note frequency table (pitch→register value mapping for all octaves)
- `wave_samples.asm` — wave channel (ch3) waveform sample data
- `headers/` — music/SFX header tables (pointers + channel assignments per song/sfx)
- `alternate_tempo.asm` — alternate tempo table used by some tracks
- `low_health_alarm.asm` — low HP warning beep
- `play_battle_music.asm` — battle music selection logic (wild, trainer, gym leader)
- `poke_flute.asm` — Poké Flute melody playback
- `pokedex_rating_sfx.asm` — Pokédex rating sound effects

## Music & SFX Data
- `music/` — ~30 BGM tracks (one .asm per song). Each uses audio macros: `channel_count`, `channel`, `tempo`, `note`, `rest`, `vibrato`, `duty_cycle`, etc. from `macros/scripts/audio.asm`
- `sfx/` — ~100+ SFX files. Same macro system. Includes `unused_fanfare.asm` and `unusedsong.asm` (legacy/unused)

## Audio Macro System
Compositions use macros from `macros/scripts/audio.asm`:
- `channel_count` — number of active channels
- `channel` — assign channel to a labeled section
- `note` / `rest` / `speed` / `tempo` — note events and timing
- `vibrato` / `pitch_slide` / `duty_cycle` — sound modulation
- `execute_music` — start playback from header

## How Audio Is Called
- `home/audio.asm` provides HOME-bank wrappers that switch to the appropriate audio engine bank
- `home/fade_audio.asm` handles fade-in/fade-out
- Music/SFX are identified by constants in `constants/music_constants.asm`
