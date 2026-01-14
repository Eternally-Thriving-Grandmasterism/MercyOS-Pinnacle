use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::sound::granular::{GranularSoundData, GranularSoundSettings, GrainOptions};
use rand::{Rng, thread_rng};

/// Granular Ambient Cloud — Infinite Evolving Drone Mercy Eternal
pub fn spawn_granular_ambient(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
    joy_level: f32,  // 0.0-10.0 philotic emotional density
    player_pos: Vec3,
) {
    let sample = asset_server.load("music/epic_dark_ambient.ogg");

    let mut rng = thread_rng();

    // Grain parameters modulated by joy_level — richer resonance eternal
    let grain_duration = 0.2 + joy_level * 0.1;  // Longer grains with higher joy
    let grains_per_sec = 10.0 + joy_level * 15.0;  // Denser clouds
    let pitch_variation = 0.9 + joy_level * 0.2;   // Brighter with joy
    let position_spread = joy_level * 5.0;        // Wider spatial joy

    let granular_settings = GranularSoundSettings {
        grain_options: GrainOptions {
            duration: Duration::Seconds(grain_duration),
            ..default()
        },
        grains_per_second: grains_per_sec,
        pitch_variation: pitch_variation..=1.2,
        position_variation: -position_spread..=position_spread,
        ..default()
    };

    let granular_sound = GranularSoundData::new(sample, granular_settings);

    audio.play(granular_sound)
        .looped()
        .with_volume(0.3 + joy_level * 0.05)
        .spatial(true)
        .with_position(player_pos);
}
