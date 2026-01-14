use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::sound::granular::{GranularSoundData, GranularSoundSettings, GrainOptions};
use crate::vector_synthesis::vector_wavetable_synthesis;
use rand::{thread_rng, Rng};
use crate::procedural_music::AdsrEnvelope;

pub fn spawn_pure_procedural_granular_ambient(
    audio: &Res<Audio>,
    joy_level: f32,
    player_pos: Vec3,
) {
    let grain_buffer_duration = 6.0 + joy_level * 4.0;
    let base_freq = 40.0 + joy_level * 50.0;

    let vector_x = joy_level * 0.5;
    let vector_y = joy_level * 0.3;

    let wavetable_grain = vector_wavetable_synthesis(grain_buffer_duration, base_freq, vector_x, vector_y, AdsrEnvelope::joy_resonance(), joy_level);

    let mut rng = thread_rng();

    let grain_duration = 0.2 + joy_level * 0.25;
    let grains_per_sec = 25.0 + joy_level * 40.0;
    let pitch_variation = 0.75 + joy_level * 0.35;

    let granular_settings = GranularSoundSettings {
        grain_options: GrainOptions {
            duration: kira::Duration::Seconds(grain_duration),
            ..default()
        },
        grains_per_second: grains_per_sec,
        pitch_variation: pitch_variation..=1.5,
        position_variation: -(joy_level * 12.0)..=(joy_level * 12.0),
        ..default()
    };

    let granular_sound = GranularSoundData::new(wavetable_grain, granular_settings);

    audio.play(granular_sound)
        .looped()
        .with_volume(0.42 + joy_level * 0.08)
        .spatial(true)
        .with_position(player_pos);
}
