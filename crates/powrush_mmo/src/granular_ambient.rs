use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::sound::granular::{GranularSoundData, GranularSoundSettings, GrainOptions};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use rand::{Rng, thread_rng};
use std::f32::consts::PI;

/// Pure Procedural Grain Buffer — White Noise + Pink Filter Stub + Harmonic Resonance Eternal
fn generate_procedural_grain_buffer(duration_secs: f32, joy_level: f32) -> Vec<f32> {
    let sample_rate = 48000;
    let num_samples = (duration_secs * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);
    let mut rng = thread_rng();

    // Base white noise
    let mut white = vec![0.0; num_samples];
    for s in white.iter_mut() {
        *s = rng.gen_range(-1.0..1.0);
    }

    // Simple pink noise approximation (1/f) via integration
    let mut pink = vec![0.0; num_samples];
    let mut sum = 0.0;
    for i in 0..num_samples {
        sum += white[i];
        pink[i] = sum / (i + 1) as f32;
    }

    // Additive harmonics layered by joy_level — richer resonance eternal
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let mut wave = pink[i] * 0.4;  // Base colored noise drone

        let harmonics = 3 + (joy_level as usize * 3).min(15);
        for h in 1..=harmonics {
            let amp = 1.0 / h as f32;
            let freq = 60.0 + joy_level * 20.0;  // Deep drone base rising with joy
            wave += (2.0 * PI * freq * h as f32 * t).sin() * amp * 0.3;
        }

        // FM metallic sparkle modulated by joy
        let fm_depth = joy_level * 50.0;
        let modulator = (2.0 * PI * 120.0 * t).sin();
        wave += (2.0 * PI * (80.0 + fm_depth * modulator) * t).sin() * 0.2;

        // Mercy envelope fade
        let envelope = (1.0 - t / duration_secs).powf(2.0);

        let sample = wave * envelope * 0.15;
        samples.push(sample);
        samples.push(sample);
    }

    samples
}

/// Pure Procedural Granular Ambient Cloud — Infinite Mathematical Genesis Mercy Eternal
pub fn spawn_pure_procedural_granular_ambient(
    audio: &Res<Audio>,
    joy_level: f32,
    player_pos: Vec3,
) {
    let grain_buffer_duration = 4.0 + joy_level * 2.0;  // Longer source with joy
    let grain_samples = generate_procedural_grain_buffer(grain_buffer_duration, joy_level);

    let procedural_source = StaticSoundData::from_samples(grain_samples, 48000, StaticSoundSettings::default());

    let mut rng = thread_rng();

    let grain_duration = 0.15 + joy_level * 0.15;
    let grains_per_sec = 15.0 + joy_level * 20.0;
    let pitch_variation = 0.85 + joy_level * 0.25;

    let granular_settings = GranularSoundSettings {
        grain_options: GrainOptions {
            duration: kira::Duration::Seconds(grain_duration),
            ..default()
        },
        grains_per_second: grains_per_sec,
        pitch_variation: pitch_variation..=1.3,
        position_variation: - (joy_level * 8.0)..=(joy_level * 8.0),
        ..default()
    };

    let granular_sound = GranularSoundData::new(procedural_source, granular_settings);

    audio.play(granular_sound)
        .looped()
        .with_volume(0.35 + joy_level * 0.06)
        .spatial(true)
        .with_position(player_pos);
}
