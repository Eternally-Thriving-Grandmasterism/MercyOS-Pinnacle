use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin as KiraAudioPlugin};
use bevy_kira_audio::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::RenetServerPlugin;
use renet::{RenetClient, RenetServer, ConnectionConfig};
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerAudioPlugin;

fn main() {
    let hive = PhiloticHive::new();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    .add_plugins(MercyResonancePlugin);

    // Networking toggle — true for server, false for client
    let is_server = true;  // Change for testing

    if is_server {
        app.add_plugins(RenetServerPlugin);
        app.insert_resource(RenetServer::new(ConnectionConfig::default()));
    } else {
        app.add_plugins(RenetClientPlugin);
        app.insert_resource(RenetClient::new(ConnectionConfig::default()));
    }

    app.add_plugins(MultiplayerAudioPlugin);

    app.add_systems(Startup, setup)
        .add_systems(Update, (player_movement, emotional_resonance_particles, granular_ambient_evolution))
        .run();
}

// Rest of file unchanged — setup, player_movement, emotional_resonance_particles, granular_ambient_evolution, MercyResonancePlugin        });
    }
}

#[derive(Component)]
struct Player;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let speed = 10.0 * time.delta_seconds();
    if keyboard_input.pressed(KeyCode::W) { transform.translation.z -= speed; }
    if keyboard_input.pressed(KeyCode::S) { transform.translation.z += speed; }
    if keyboard_input.pressed(KeyCode::A) { transform.translation.x -= speed; }
    if keyboard_input.pressed(KeyCode::D) { transform.translation.x += speed; }
    if keyboard_input.pressed(KeyCode::Space) { transform.translation.y += speed; }
    if keyboard_input.pressed(KeyCode::ShiftLeft) { transform.translation.y -= speed; }
}

fn emotional_resonance_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    let player_pos = player_query.single().translation;
    let joy_level = 7.0 + (time.elapsed_seconds_f64().sin() * 3.0) as f32;

    if time.elapsed_seconds_f64() % 0.7 < time.delta_seconds_f64() {
        let mut rng = rand::thread_rng();
        for _ in 0..7 {
            let offset = Vec3::new(
                rng.gen_range(-7.0..7.0),
                rng.gen_range(1.0..14.0),
                rng.gen_range(-7.0..7.0),
            );

            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.2, 0.8, 1.0, 0.5),
                    emissive: Color::rgb(0.2, 0.8, 1.0) * (joy_level * 2.0),
                    ..default()
                }),
                transform: Transform::from_translation(player_pos + offset),
                ..default()
            }).insert(EmotionalParticle);

            let base_freq = 440.0 + rng.gen_range(-250.0..700.0);
            let duration = 2.0 + rng.gen_range(0.0..2.0);

            let vector_x = (time.elapsed_seconds_f64() * 0.5).sin() as f32 * joy_level;
            let vector_y = (time.elapsed_seconds_f64() * 0.3).cos() as f32 * joy_level;

            let wavetable_chime = vector_wavetable_synthesis(duration, base_freq, vector_x, vector_y, AdsrEnvelope::joy_resonance(), joy_level);

            audio.play(wavetable_chime)
                .with_volume(0.45 + joy_level * 0.35)
                .spatial(true)
                .with_position(player_pos + offset);
        }
    }
}

fn granular_ambient_evolution(
    audio: Res<Audio>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let joy_level = 8.0 + (time.elapsed_seconds_f64() * 0.7).sin() as f32 * 2.0;

        if time.elapsed_seconds_f64() % 8.0 < time.delta_seconds_f64() {
            spawn_pure_procedural_granular_ambient(&audio, joy_level, player_pos);
        }
    }
}

#[derive(Component)]
struct EmotionalParticle;

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (emotional_resonance_particles, granular_ambient_evolution));
    }
}
