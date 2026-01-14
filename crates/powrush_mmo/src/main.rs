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
        .add_systems(Update, (player_movement, emotional_resonance_particles, granular_ambient_evolution, remote_interpolation))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(42);

    let ground_mesh = meshes.add(shape::Plane::from_size(1000.0).into());
    let ground_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

    commands.spawn(PbrBundle {
        mesh: ground_mesh,
        material: ground_material,
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Player,
        Predicted,
        Velocity(Vec3::ZERO),
        PositionHistory { buffer: VecDeque::new() },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        ..default()
    });

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(-500.0..500.0);
        let z = rng.gen_range(-500.0..500.0);
        let y = perlin.get([x as f64 / 100.0, z as f64 / 100.0]) as f32 * 5.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            material: materials.add(Color::rgb(1.0, 0.8, 0.2).into()),
            transform: Transform::from_xyz(x, y + 2.0, z),
            ..default()
        });
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Predicted;

#[derive(Component)]
struct Velocity(pub Vec3);

#[derive(Component)]
struct PositionHistory {
    pub buffer: VecDeque<(Vec3, f64)>,
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<Player>, With<Predicted>)>,
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

fn remote_interpolation(
    mut query: Query<(&mut Transform, &PositionHistory), Without<Predicted>>,
    time: Res<Time>,
) {
    let render_time = time.elapsed_seconds_f64() - 0.1;  // 100ms delay mercy

    for (mut transform, history) in &mut query {
        if history.buffer.len() >= 2 {
            let mut older = history.buffer[0];
            let mut newer = history.buffer[history.buffer.len() - 1];

            for &(pos, ts) in &history.buffer {
                if ts <= render_time {
                    older = (pos, ts);
                } else {
                    newer = (pos, ts);
                    break;
                }
            }

            let t = ((render_time - older.1) / (newer.1 - older.1).max(0.001)) as f32;
            transform.translation = older.0.lerp(newer.0, t.clamp(0.0, 1.0));
        }
    }
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
        app.add_systems(Update, (emotional_resonance_particles, granular_ambient_evolution, remote_interpolation));
    }
}            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        ..default()
    });

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(-500.0..500.0);
        let z = rng.gen_range(-500.0..500.0);
        let y = perlin.get([x as f64 / 100.0, z as f64 / 100.0]) as f32 * 5.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            material: materials.add(Color::rgb(1.0, 0.8, 0.2).into()),
            transform: Transform::from_xyz(x, y + 2.0, z),
            ..default()
        });
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Predicted;

#[derive(Component)]
struct Velocity(pub Vec3);

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<Player>, With<Predicted>)>,
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

        // Immediate prediction mercy
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn dead_reckoning(
    mut query: Query<(&mut Transform, &Velocity), Without<Predicted>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_seconds();
    }
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
        app.add_systems(Update, (emotional_resonance_particles, granular_ambient_evolution, dead_reckoning));
    }
}
