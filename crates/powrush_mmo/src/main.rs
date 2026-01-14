use bevy::prelude::*;
use bevy::render::view::Visibility;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin as KiraAudioPlugin};
use bevy_kira_audio::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::RenetServerPlugin;
use renet::{RenetClient, RenetServer, ConnectionConfig};
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin, Seedable};
use ndshape::{ConstShape3u32, ConstArray};
use greedly::{GreedyMesher, Face, Quad};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use bevy_rapier3d::prelude::*;
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;

const CHUNK_SIZE: u32 = 32;
const VIEW_CHUNKS: i32 = 5;

#[derive(Component)]
struct Chunk {
    coord: IVec2,
}

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
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
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
        .add_systems(Update, (
            player_movement,
            emotional_resonance_particles,
            granular_ambient_evolution,
            chunk_manager,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Local player with physics
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 30.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        Player,
        Predicted,
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        PositionHistory { buffer: VecDeque::new() },
    ));
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
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::S) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::A) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::D) { direction.x += 1.0; }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let speed = 10.0;
        velocity.linvel = Vec3::new(direction.x * speed, velocity.linvel.y, direction.z * speed);
    }
}

fn chunk_manager(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(Entity, &Chunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let player_chunk = IVec2::new(
            (player_pos.x / CHUNK_SIZE as f32).floor() as i32,
            (player_pos.z / CHUNK_SIZE as f32).floor() as i32,
        );

        let stone_mat = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
        let dirt_mat = materials.add(Color::rgb(0.6, 0.4, 0.2).into());
        let grass_mat = materials.add(Color::rgb(0.2, 0.7, 0.2).into());

        for dx in -VIEW_CHUNKS..=VIEW_CHUNKS {
            for dz in -VIEW_CHUNKS..=VIEW_CHUNKS {
                let chunk_coord = player_chunk + IVec2::new(dx, dz);
                let chunk_exists = chunk_query.iter().any(|(_, chunk)| chunk.coord == chunk_coord);

                if !chunk_exists {
                    let seed = ((chunk_coord.x as u64) << 32) | chunk_coord.y as u64;
                    let perlin = Perlin::new(seed as u32);

                    let mut voxels = [0u8; ChunkShape::SIZE as usize];

                    // Voxel generation logic unchanged...

                    // Greedy meshing + collider
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                    // Full greedy implementation stubbed — add vertices/indices from quads

                    let chunk_mesh = meshes.add(mesh);

                    commands.spawn((
                        PbrBundle {
                            mesh: chunk_mesh.clone(),
                            material: grass_mat.clone(),
                            transform: Transform::from_xyz(chunk_coord.x as f32 * CHUNK_SIZE as f32, 0.0, chunk_coord.y as f32 * CHUNK_SIZE as f32),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        Chunk { coord: chunk_coord },
                        Collider::trimesh_from_mesh(&chunk_mesh).unwrap(),
                        RigidBody::Fixed,
                    ));
                }
            }
        }

        // Despawn far chunks unchanged...
    }
}

// Other systems unchanged...

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            chunk_manager,
        ));
    }
}
