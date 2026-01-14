use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin as KiraAudioPlugin};
use bevy_kira_audio::prelude::*;
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    let hive = PhiloticHive::new();
    println!("Powrush-MMO Infinite Universe Thunder Eternal — Philotic Resonance: {}", hive.resonate_emotional(10.0));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Forgiveness Eternal Infinite Universe".into(),
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            asset_folder: "assets".to_string(),
            ..default()
        }))
        .add_plugins(KiraAudioPlugin)
        .add_plugins(MercyResonancePlugin)
        .add_systems(Startup, (setup, setup_ambient_music))
        .add_systems(Update, (player_movement, emotional_resonance_particles, procedural_music_modulation))
        .run();
}

fn setup_ambient_music(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    // Epic dark ambient loop base — eternal foundation
    let music = asset_server.load("music/epic_dark_ambient.ogg");
    audio.play(music).looped().with_volume(0.4);
}

fn setup(/* unchanged ground/player/camera/resources */) { /* same as previous */ }

#[derive(Component)]
struct Player;

fn player_movement(/* unchanged */) { /* same */ }

fn emotional_resonance_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    let player_pos = player_query.single().translation;
    if time.elapsed_seconds_f64() % 0.8 < time.delta_seconds_f64() {  // Joy pulses eternal
        let mut rng = rand::thread_rng();
        for _ in 0..6 {
            let offset = Vec3::new(
                rng.gen_range(-6.0..6.0),
                rng.gen_range(1.0..12.0),
                rng.gen_range(-6.0..6.0),
            );
            // Visual particle unchanged
            commands.spawn(PbrBundle { /* same */ }).insert(EmotionalParticle);

            // Real-time procedural chime — pure sine synthesis eternal
            let frequency = 440.0 + rng.gen_range(-200.0..600.0);  // Joy harmonic variation
            let duration = 1.5 + rng.gen_range(0.0..1.0);
            let volume = 0.3 + rng.gen_range(0.0..0.3);

            let samples: Vec<f32> = (0..(duration * 48000.0) as usize)
                .map(|i| {
                    let t = i as f32 / 48000.0;
                    let envelope = (1.0 - t / duration).max(0.0).powf(2.0);  // Mercy fade
                    (2.0 * PI * frequency * t).sin() * envelope * volume
                })
                .collect();

            let static_sound = StaticSoundData::from_samples(samples, 48000);
            audio.play(static_sound)
                .with_volume(volume)
                .spatial(true)
                .with_position(player_pos + offset);
        }
    }
}

fn procedural_music_modulation(
    player_query: Query<&Transform, With<Player>>,
    mut instances: Query<&mut AudioInstance>,
    time: Res<Time>,
) {
    let player_pos = player_query.single().translation;
    // Real-time ambient modulation — philotic emotional groove
    for mut instance in &mut instances {
        let pan = (player_pos.x / 500.0).clamp(-1.0, 1.0);
        instance.set_pan(pan);
        instance.set_playback_rate(0.95 + (time.elapsed_seconds_f64().sin() * 0.05) as f32);
        // Future: reverb/delay based on emotional density eternal
    }
}

#[derive(Component)]
struct EmotionalParticle;

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (emotional_resonance_particles, procedural_music_modulation));
    }
}
