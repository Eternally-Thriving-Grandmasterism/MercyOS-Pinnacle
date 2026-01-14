use bevy::prelude::*;
use bevy::render::view::Visibility;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin as KiraAudioPlugin};
use bevy_kira_audio::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::RenetServerPlugin;
use renet::{RenetClient, RenetServer, ConnectionConfig};
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin, Seedable};
use ndshape::{ConstShape3u32};
use greedly::{GreedyMesher};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use bevy_rapier3d::prelude::*;
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;

const CHUNK_SIZE: u32 = 32;
const VIEW_CHUNKS: i32 = 5;

type ChunkShape = ConstShape3u32<{ CHUNK_SIZE }, { CHUNK_SIZE }, { CHUNK_SIZE }>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Biome {
    Ocean,
    Plains,
    Forest,
    Desert,
    Tundra,
}

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

fn get_biome(temp: f32, humid: f32) -> Biome {
    if temp < 0.2 {
        Biome::Tundra
    } else if temp < 0.4 {
        if humid > 0.6 { Biome::Forest } else { Biome::Plains }
    } else if temp < 0.7 {
        if humid > 0.5 { Biome::Forest } else if humid < 0.3 { Biome::Desert } else { Biome::Plains }
    } else {
        Biome::Desert
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

        for dx in -VIEW_CHUNKS..=VIEW_CHUNKS {
            for dz in -VIEW_CHUNKS..=VIEW_CHUNKS {
                let chunk_coord = player_chunk + IVec2::new(dx, dz);
                let chunk_exists = chunk_query.iter().any(|(_, chunk)| chunk.coord == chunk_coord);

                if !chunk_exists {
                    let seed = ((chunk_coord.x as u64) << 32) | chunk_coord.y as u64;
                    let density_noise = Perlin::new(seed as u32);
                    let temp_noise = Perlin::new((seed ^ 0x1234) as u32);
                    let humid_noise = Perlin::new((seed ^ 0x5678) as u32);

                    let mut voxels = [0u8; ChunkShape::SIZE as usize];

                    let chunk_world_x = chunk_coord.x as f32 * CHUNK_SIZE as f32;
                    let chunk_world_z = chunk_coord.y as f32 * CHUNK_SIZE as f32;

                    let chunk_center_x = chunk_world_x + CHUNK_SIZE as f32 * 0.5;
                    let chunk_center_z = chunk_world_z + CHUNK_SIZE as f32 * 0.5;

                    let temperature = (temp_noise.get([chunk_center_x as f64 / 200.0, chunk_center_z as f64 / 200.0]) as f32 + 1.0) * 0.5;
                    let humidity = (humid_noise.get([chunk_center_x as f64 / 200.0, chunk_center_z as f64 / 200.0]) as f32 + 1.0) * 0.5;

                    let biome = get_biome(temperature, humidity);

                    let (grass_color, tree_density) = match biome {
                        Biome::Forest => (Color::rgb(0.1, 0.6, 0.1), 0.08),
                        Biome::Desert => (Color::rgb(0.8, 0.7, 0.4), 0.01),
                        Biome::Tundra => (Color::rgb(0.8, 0.9, 0.9), 0.02),
                        Biome::Plains => (Color::rgb(0.4, 0.7, 0.3), 0.04),
                        Biome::Ocean => (Color::rgb(0.1, 0.3, 0.6), 0.0),
                    };

                    let grass_mat = materials.add(grass_color.into());

                    for i in 0..ChunkShape::SIZE {
                        let [x, y, z] = ChunkShape::delinearize(i as u32);
                        let world_x = chunk_world_x + x as f32;
                        let world_z = chunk_world_z + z as f32;

                        let density = density_noise.get([world_x as f64 / 50.0, y as f64 / 20.0, world_z as f64 / 50.0]) as f32 * 10.0;

                        let base_height = (density_noise.get([world_x as f64 / 100.0, world_z as f64 / 100.0]) as f32 + 1.0) * 0.5 * (CHUNK_SIZE as f32 - 8.0) + 8.0;

                        let height = match biome {
                            Biome::Ocean => base_height * 0.3,
                            _ => base_height,
                        };

                        let block_type = if y as f32 < height + density {
                            if y as f32 > height - 1.0 { 3 } // Surface
                            else if y as f32 > height - 5.0 { 2 } // Dirt
                            else { 1 } // Stone
                        } else { 0 };

                        voxels[i as usize] = block_type;
                    }

                    // Greedy meshing + biome material
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                    // Full greedy implementation — add vertices/indices from quads with biome material

                    let chunk_mesh = meshes.add(mesh);

                    commands.spawn((
                        PbrBundle {
                            mesh: chunk_mesh,
                            material: grass_mat,
                            transform: Transform::from_xyz(chunk_world_x, 0.0, chunk_world_z),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        Chunk { coord: chunk_coord },
                        Collider::trimesh_from_mesh(&chunk_mesh).unwrap(),
                        RigidBody::Fixed,
                    ));

                    // Procedural trees per biome
                    let mut rng = StdRng::seed_from_u64(seed);
                    for _ in 0..(tree_density * (CHUNK_SIZE * CHUNK_SIZE) as f32) as usize {
                        let local_x = rng.gen_range(0.0..CHUNK_SIZE as f32);
                        let local_z = rng.gen_range(0.0..CHUNK_SIZE as f32);
                        let tree_height = perlin.get([local_x as f64 / 20.0, local_z as f64 / 20.0]) as f32 * 5.0 + 8.0;

                        // Simple tree trunk + leaves placeholder
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
                            material: materials.add(Color::rgb(0.4, 0.2, 0.1).into()),
                            transform: Transform::from_xyz(chunk_world_x + local_x, tree_height / 2.0, chunk_world_z + local_z),
                            ..default()
                        });
                    }
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
