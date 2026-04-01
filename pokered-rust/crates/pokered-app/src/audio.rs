use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use pokered_audio::audio_manager::AudioManager;
use pokered_audio::music_data::MusicId;
use pokered_audio::sfx_data::SfxId;
use pokered_audio::CPU_CLOCK_HZ;
use pokered_data::species::Species;

/// Map a Species to its base cry SfxId.
///
/// Derived from `data/pokemon/cries.asm`. Each species has a base cry
/// (SFX_CRY_00 through SFX_CRY_25) plus pitch/length modifiers. Since our
/// audio engine plays the base cry without per-species modifications, this
/// returns the base cry SfxId only.
pub fn species_to_cry(species: Species) -> SfxId {
    match species {
        Species::Bulbasaur => SfxId::Cry0F,
        Species::Ivysaur => SfxId::Cry0F,
        Species::Venusaur => SfxId::Cry0F,
        Species::Charmander => SfxId::Cry04,
        Species::Charmeleon => SfxId::Cry04,
        Species::Charizard => SfxId::Cry04,
        Species::Squirtle => SfxId::Cry1D,
        Species::Wartortle => SfxId::Cry1D,
        Species::Blastoise => SfxId::Cry13,
        Species::Caterpie => SfxId::Cry16,
        Species::Metapod => SfxId::Cry1C,
        Species::Butterfree => SfxId::Cry16,
        Species::Weedle => SfxId::Cry15,
        Species::Kakuna => SfxId::Cry13,
        Species::Beedrill => SfxId::Cry13,
        Species::Pidgey => SfxId::Cry0E,
        Species::Pidgeotto => SfxId::Cry14,
        Species::Pidgeot => SfxId::Cry14,
        Species::Rattata => SfxId::Cry22,
        Species::Raticate => SfxId::Cry22,
        Species::Spearow => SfxId::Cry10,
        Species::Fearow => SfxId::Cry18,
        Species::Ekans => SfxId::Cry17,
        Species::Arbok => SfxId::Cry17,
        Species::Pikachu => SfxId::Cry0F,
        Species::Raichu => SfxId::Cry09,
        Species::Sandshrew => SfxId::Cry00,
        Species::Sandslash => SfxId::Cry00,
        Species::NidoranF => SfxId::Cry01,
        Species::Nidorina => SfxId::Cry01,
        Species::Nidoqueen => SfxId::Cry0A,
        Species::NidoranM => SfxId::Cry00,
        Species::Nidorino => SfxId::Cry00,
        Species::Nidoking => SfxId::Cry09,
        Species::Clefairy => SfxId::Cry19,
        Species::Clefable => SfxId::Cry19,
        Species::Vulpix => SfxId::Cry24,
        Species::Ninetales => SfxId::Cry24,
        Species::Jigglypuff => SfxId::Cry0E,
        Species::Wigglytuff => SfxId::Cry0E,
        Species::Zubat => SfxId::Cry1D,
        Species::Golbat => SfxId::Cry1D,
        Species::Oddish => SfxId::Cry08,
        Species::Gloom => SfxId::Cry08,
        Species::Vileplume => SfxId::Cry23,
        Species::Paras => SfxId::Cry1E,
        Species::Parasect => SfxId::Cry1E,
        Species::Venonat => SfxId::Cry1A,
        Species::Venomoth => SfxId::Cry1A,
        Species::Diglett => SfxId::Cry0B,
        Species::Dugtrio => SfxId::Cry0B,
        Species::Meowth => SfxId::Cry19,
        Species::Persian => SfxId::Cry19,
        Species::Psyduck => SfxId::Cry21,
        Species::Golduck => SfxId::Cry21,
        Species::Mankey => SfxId::Cry0A,
        Species::Primeape => SfxId::Cry0A,
        Species::Growlithe => SfxId::Cry1F,
        Species::Arcanine => SfxId::Cry15,
        Species::Poliwag => SfxId::Cry0E,
        Species::Poliwhirl => SfxId::Cry0E,
        Species::Poliwrath => SfxId::Cry0E,
        Species::Abra => SfxId::Cry1C,
        Species::Kadabra => SfxId::Cry1C,
        Species::Alakazam => SfxId::Cry1C,
        Species::Machop => SfxId::Cry1F,
        Species::Machoke => SfxId::Cry1F,
        Species::Machamp => SfxId::Cry1F,
        Species::Bellsprout => SfxId::Cry21,
        Species::Weepinbell => SfxId::Cry25,
        Species::Victreebel => SfxId::Cry25,
        Species::Tentacool => SfxId::Cry1A,
        Species::Tentacruel => SfxId::Cry1A,
        Species::Geodude => SfxId::Cry24,
        Species::Graveler => SfxId::Cry24,
        Species::Golem => SfxId::Cry12,
        Species::Ponyta => SfxId::Cry25,
        Species::Rapidash => SfxId::Cry25,
        Species::Slowpoke => SfxId::Cry02,
        Species::Slowbro => SfxId::Cry1F,
        Species::Magnemite => SfxId::Cry1C,
        Species::Magneton => SfxId::Cry1C,
        Species::Farfetchd => SfxId::Cry10,
        Species::Doduo => SfxId::Cry0B,
        Species::Dodrio => SfxId::Cry0B,
        Species::Seel => SfxId::Cry0C,
        Species::Dewgong => SfxId::Cry0C,
        Species::Grimer => SfxId::Cry05,
        Species::Muk => SfxId::Cry07,
        Species::Shellder => SfxId::Cry18,
        Species::Cloyster => SfxId::Cry18,
        Species::Gastly => SfxId::Cry1C,
        Species::Haunter => SfxId::Cry1C,
        Species::Gengar => SfxId::Cry07,
        Species::Onix => SfxId::Cry17,
        Species::Drowzee => SfxId::Cry0D,
        Species::Hypno => SfxId::Cry0D,
        Species::Krabby => SfxId::Cry20,
        Species::Kingler => SfxId::Cry20,
        Species::Voltorb => SfxId::Cry06,
        Species::Electrode => SfxId::Cry06,
        Species::Exeggcute => SfxId::Cry0B,
        Species::Exeggutor => SfxId::Cry0D,
        Species::Cubone => SfxId::Cry19,
        Species::Marowak => SfxId::Cry08,
        Species::Hitmonlee => SfxId::Cry12,
        Species::Hitmonchan => SfxId::Cry0C,
        Species::Lickitung => SfxId::Cry0C,
        Species::Koffing => SfxId::Cry12,
        Species::Weezing => SfxId::Cry12,
        Species::Rhyhorn => SfxId::Cry04,
        Species::Rhydon => SfxId::Cry11,
        Species::Chansey => SfxId::Cry14,
        Species::Tangela => SfxId::Cry12,
        Species::Kangaskhan => SfxId::Cry03,
        Species::Horsea => SfxId::Cry19,
        Species::Seadra => SfxId::Cry19,
        Species::Goldeen => SfxId::Cry16,
        Species::Seaking => SfxId::Cry16,
        Species::Staryu => SfxId::Cry1E,
        Species::Starmie => SfxId::Cry1E,
        Species::MrMime => SfxId::Cry20,
        Species::Scyther => SfxId::Cry16,
        Species::Jynx => SfxId::Cry0D,
        Species::Electabuzz => SfxId::Cry06,
        Species::Magmar => SfxId::Cry04,
        Species::Pinsir => SfxId::Cry14,
        Species::Tauros => SfxId::Cry1D,
        Species::Magikarp => SfxId::Cry17,
        Species::Gyarados => SfxId::Cry17,
        Species::Lapras => SfxId::Cry1B,
        Species::Ditto => SfxId::Cry0E,
        Species::Eevee => SfxId::Cry1A,
        Species::Vaporeon => SfxId::Cry1A,
        Species::Jolteon => SfxId::Cry1A,
        Species::Flareon => SfxId::Cry1A,
        Species::Porygon => SfxId::Cry25,
        Species::Omanyte => SfxId::Cry1F,
        Species::Omastar => SfxId::Cry1F,
        Species::Kabuto => SfxId::Cry16,
        Species::Kabutops => SfxId::Cry18,
        Species::Aerodactyl => SfxId::Cry23,
        Species::Snorlax => SfxId::Cry05,
        Species::Articuno => SfxId::Cry09,
        Species::Zapdos => SfxId::Cry18,
        Species::Moltres => SfxId::Cry09,
        Species::Dratini => SfxId::Cry0F,
        Species::Dragonair => SfxId::Cry0F,
        Species::Dragonite => SfxId::Cry0F,
        Species::Mewtwo => SfxId::Cry1E,
        Species::Mew => SfxId::Cry1E,
        Species::None => SfxId::Cry00,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct AudioOutput {
    pub manager: Arc<Mutex<AudioManager>>,
    pub _stream: cpal::Stream,
}

#[cfg(not(target_arch = "wasm32"))]
impl AudioOutput {
    pub fn new() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(44_100),
            buffer_size: cpal::BufferSize::Default,
        };

        let manager = Arc::new(Mutex::new(AudioManager::new()));
        // Enable APU power (NR52 bit 7). Without this, all APU register writes
        // are silently ignored and no sound is produced.
        {
            let mut mgr = manager.lock().unwrap();
            mgr.apu.write_register(0xFF26, 0x80);
        }
        let mgr_clone = Arc::clone(&manager);

        let cycles_per_sample = CPU_CLOCK_HZ / 44_100;
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut mgr = mgr_clone.lock().unwrap();
                    let max_amplitude = 480.0_f32;
                    for frame in data.chunks_mut(2) {
                        mgr.apu.tick_n(cycles_per_sample);
                        let (left, right) = mgr.apu.mix_sample();
                        frame[0] = left as f32 / max_amplitude;
                        frame[1] = right as f32 / max_amplitude;
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
            .ok()?;

        stream.play().ok()?;

        Some(Self {
            manager,
            _stream: stream,
        })
    }

    pub fn play_music(&self, id: MusicId) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.play_music(id);
        }
    }

    pub fn play_sfx(&self, id: SfxId) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.play_sfx(id);
        }
    }

    pub fn stop_music(&self) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.stop_music();
        }
    }

    pub fn stop_all(&self) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.stop_all();
        }
    }

    pub fn last_music_id(&self) -> Option<MusicId> {
        if let Ok(mgr) = self.manager.lock() {
            mgr.last_music_id()
        } else {
            None
        }
    }

    pub fn update_frame(&self) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.update_frame();
        }
    }
}
