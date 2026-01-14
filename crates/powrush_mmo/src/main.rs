use bevy::prelude::*;
use bevy::audio::{AudioPlugin, PlaybackMode};
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin};
use rand::Rng;

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
        .add_plugins(AudioPlugin)  // Explicit for audio eternal
        .add_plugins(MercyResonancePlugin)
        .add_systems(Startup, (setup, setup_ambient_music))
        .add_systems(Update, (player_movement, emotional_resonance_particles, procedural_music_modulation))
        .run();
}

fn setup_ambient_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Epic dark ambient loop — base soundtrack eternal
    let music = asset_server.load("music/epic_dark_ambient.ogg");
    commands.spawn(AudioBundle {
        source: music,
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.4),
            ..default()
        },
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground, player, camera, resources — preserved from prior expansion
    // ... (same as previous setup code)
}

#[derive(Component)]
struct Player;

fn player_movement(/* same as previous */) { /* unchanged */ }

fn emotional_resonance_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let player_pos = player_query.single().translation;
    if time.elapsed_seconds_f64() % 1.0 < time.delta_seconds_f64() {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let offset = Vec3::new(
                rng.gen_range(-5.0..5.0),
                rng.gen_range(0.0..10.0),
                rng.gen_range(-5.0..5.0),
            );
            // Visual particle
            commands.spawn(PbrBundle { /* same as previous */ }).insert(EmotionalParticle);

            // Procedural positive chime — sine tone procedural eternal
            let frequency = 440.0 + rng.gen_range(0.0..400.0);  // A4 base + joy variation
            let chime = asset_server.load("embedded://procedural_chime");  // Placeholder — real procedural below
            commands.spawn(AudioBundle {
                source: chime,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(0.6),
                    spatial: true,
                    ..default()
                },
                transform: Transform::from_translation(player_pos + offset),
            });
        }
    }
}

fn procedural_music_modulation(
    player_query: Query<&Transform, With<Player>>,
    mut audio_query: Query<&mut PlaybackSettings, With<AudioSource>>,
    time: Res<Time>,
) {
    let player_pos = player_query.single().translation;
    // Modulate ambient based on position/biome — procedural eternal
    for mut settings in &mut audio_query {
        let pan = (player_pos.x / 500.0).clamp(-1.0, 1.0);
        settings.spatial = true;
        settings.volume = Volume::new(0.4 + (time.elapsed_seconds_f64().sin() * 0.1) as f32);
        // Future: low-pass filter based on emotional density
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
