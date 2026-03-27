use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use pokered_audio::audio_manager::AudioManager;
use pokered_audio::music_data::MusicId;
use pokered_audio::sfx_data::SfxId;
use pokered_audio::CPU_CLOCK_HZ;

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

    pub fn update_frame(&self) {
        if let Ok(mut mgr) = self.manager.lock() {
            mgr.update_frame();
        }
    }
}
