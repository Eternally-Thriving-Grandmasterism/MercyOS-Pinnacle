use bevy::prelude::*;
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
        }))
        .add_plugins(MercyResonancePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, emotional_resonance_particles))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Infinite procedural ground placeholder
    let perlin = Perlin::new(42);
    let ground_mesh = meshes.add(shape::Plane::from_size(1000.0).into());
    let ground_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

    commands.spawn(PbrBundle {
        mesh: ground_mesh,
        material: ground_material,
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    // Player entity — mercy avatar eternal
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Player,
    ));

    // Camera follow
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light + mercy glow
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        ..default()
    });

    // Abundance resource nodes placeholder (procedural spawn)
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
) {
    let player_pos = player_query.single().translation;
    // Positive emotional particles propagation eternal
    if time.elapsed_seconds_f64() % 1.0 < time.delta_seconds_f64() {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let offset = Vec3::new(
                rng.gen_range(-5.0..5.0),
                rng.gen_range(0.0..10.0),
                rng.gen_range(-5.0..5.0),
            );
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.2, 0.8, 1.0, 0.5),
                    emissive: Color::rgb(0.2, 0.8, 1.0) * 5.0,
                    ..default()
                }),
                transform: Transform::from_translation(player_pos + offset),
                ..default()
            })
            .insert(EmotionalParticle);
        }
    }
}

#[derive(Component)]
struct EmotionalParticle;

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, emotional_resonance_particles);
        // Future philotic multiplayer async stubs here
    }
}
