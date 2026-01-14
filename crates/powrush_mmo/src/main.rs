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
use crate::networking::MultiplayerReplicationPlugin;

fn main() {
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
    .add_plugins(MultiplayerReplicationPlugin);

    let is_server = true;

    if is_server {
        app.add_plugins(RenetServerPlugin);
        app.insert_resource(RenetServer::new(ConnectionConfig::default()));
    } else {
        app.add_plugins(RenetClientPlugin);
        app.insert_resource(RenetClient::new(ConnectionConfig::default()));
    }

    app.add_systems(Startup, setup)
        .add_systems(Update, (player_movement, emotional_resonance_particles, granular_ambient_evolution, dead_reckoning))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground, light unchanged...

    // Local player spawn
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Player,
        Velocity(Vec3::ZERO),
    ));

    // Resources unchanged...
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::S) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::A) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::D) { direction.x += 1.0; }
        if keyboard_input.pressed(KeyCode::Space) { direction.y += 1.0; }
        if keyboard_input.pressed(KeyCode::ShiftLeft) { direction.y -= 1.0; }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let speed = 10.0;
        velocity.0 = direction * speed;
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn dead_reckoning(
    mut query: Query<(&mut Transform, &Velocity), Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

// emotional_resonance_particles, granular_ambient_evolution, MercyResonancePlugin unchanged
