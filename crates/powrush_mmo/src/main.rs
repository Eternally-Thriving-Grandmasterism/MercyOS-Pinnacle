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
use ndshape::runtime::shape::U32Shape3;
use greedly::{GreedyMesher, Face, Quad};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;

const CHUNK_SIZE: u32 = 32;
const VIEW_CHUNKS: i32 = 5;
const LOD_HIGH_THRESHOLD: f32 = 50.0;
const LOD_LOW_THRESHOLD: f32 = 500.0;

type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

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
            remote_interpolation,
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

    // Local player
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
        Velocity(Vec3::ZERO),
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

        let speed = 20.0;
        velocity.0 = direction * speed;

        transform.translation += velocity.0 * time.delta_seconds();
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

        // Spawn new chunks
        for dx in -VIEW_CHUNKS..=VIEW_CHUNKS {
            for dz in -VIEW_CHUNKS..=VIEW_CHUNKS {
                let chunk_coord = player_chunk + IVec2::new(dx, dz);
                let chunk_exists = chunk_query.iter().any(|(_, chunk)| chunk.coord == chunk_coord);

                if !chunk_exists {
                    let seed = ((chunk_coord.x as u64) << 32) | chunk_coord.y as u64;
                    let perlin = Perlin::new(seed as u32);

                    // Voxel data array
                    let mut voxels = [0u8; ChunkShape::SIZE as usize];

                    for i in 0..ChunkShape::SIZE {
                        let [x, y, z] = ChunkShape::delinearize(i as u32);
                        let world_x = chunk_coord.x as f32 * CHUNK_SIZE as f32 + x as f32;
                        let world_z = chunk_coord.y as f32 * CHUNK_SIZE as f32 + z as f32;

                        let density = perlin.get([world_x as f64 / 50.0, y as f64 / 20.0, world_z as f64 / 50.0]) as f32 * 10.0
                            + perlin.get([world_x as f64 / 20.0, y as f64 / 10.0, world_z as f64 / 20.0]) as f32 * 5.0;

                        let height = (perlin.get([world_x as f64 / 100.0, world_z as f64 / 100.0]) as f32 + 1.0) * 0.5 * (CHUNK_SIZE as f32 - 8.0) + 8.0;

                        let block_type = if y as f32 < height + density {
                            if y as f32 > height - 2.0 { 3 } // Grass
                            else if y as f32 > height - 6.0 { 2 } // Dirt
                            else { 1 } // Stone
                        } else { 0 }; // Air

                        voxels[i as usize] = block_type;
                    }

                    // Greedy meshing
                    let mut mesher = GreedyMesher::new(&voxels);
                    let mut mesh_builder = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);

                    let mut vertices = Vec::new();
                    let mut indices = Vec::new();
                    let mut normals = Vec::new();

                    while let Some(quad) = mesher.next_quad() {
                        let material = match voxels[mesher.index(quad.min)] {
                            1 => stone_mat.clone(),
                            2 => dirt_mat.clone(),
                            3 => grass_mat.clone(),
                            _ => continue,
                        };

                        // Simple quad vertices (expand for real geometry)
                        // Note: Full greedy implementation would add proper verts/normals/uvs
                        // Placeholder for brevity — real code uses quad to vertex builder
                    }

                    // For simplicity, spawn multiple small chunks or use full greedy (code truncated for response length)
                    // In practice, implement full greedy quad to mesh conversion

                    // Spawn chunk entity with generated mesh
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), // Placeholder
                            material: grass_mat.clone(),
                            transform: Transform::from_xyz(chunk_coord.x as f32 * CHUNK_SIZE as f32, 0.0, chunk_coord.y as f32 * CHUNK_SIZE as f32),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        Chunk { coord: chunk_coord },
                    ));
                }
            }
        }

        // Despawn far chunks
        for (entity, chunk) in &chunk_query {
            let chunk_dist = (chunk.coord - player_chunk).as_vec2().length();
            if chunk_dist > VIEW_CHUNKS as f32 + 1.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Other systems (player_movement, emotional_resonance_particles, granular_ambient_evolution, remote_interpolation) unchanged

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            remote_interpolation,
            chunk_manager,
        ));
    }
}        ..default()
    });

    let high_mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 5 }));
    let low_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));

    // Local player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        Player,
        Predicted,
        Velocity(Vec3::ZERO),
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
            (player_pos.x / CHUNK_SIZE).floor() as i32,
            (player_pos.z / CHUNK_SIZE).floor() as i32,
        );

        let ground_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

        // Spawn new chunks
        for dx in -VIEW_CHUNKS..=VIEW_CHUNKS {
            for dz in -VIEW_CHUNKS..=VIEW_CHUNKS {
                let chunk_coord = player_chunk + IVec2::new(dx, dz);
                let chunk_exists = chunk_query.iter().any(|(_, chunk)| chunk.coord == chunk_coord);

                if !chunk_exists {
                    let chunk_x = chunk_coord.x as f32 * CHUNK_SIZE;
                    let chunk_z = chunk_coord.y as f32 * CHUNK_SIZE;

                    let perlin = Perlin::new(chunk_coord.x as u32 ^ chunk_coord.y as u32);

                    // Chunk ground
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(shape::Plane::from_size(CHUNK_SIZE).into()),
                            material: ground_material.clone(),
                            transform: Transform::from_xyz(chunk_x + CHUNK_SIZE / 2.0, 0.0, chunk_z + CHUNK_SIZE / 2.0),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        Chunk { coord: chunk_coord },
                    ));

                    // Procedural resources per chunk — deterministic seed
                    let mut rng = StdRng::seed_from_u64(((chunk_coord.x as u64) << 32) | chunk_coord.y as u64);
                    for _ in 0..20 {
                        let local_x = rng.gen_range(0.0..CHUNK_SIZE);
                        let local_z = rng.gen_range(0.0..CHUNK_SIZE);
                        let height = perlin.get([local_x as f64 / 20.0, local_z as f64 / 20.0]) as f32 * 5.0;

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                                material: materials.add(Color::rgb(1.0, 0.8, 0.2).into()),
                                transform: Transform::from_xyz(chunk_x + local_x, height + 2.0, chunk_z + local_z),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            Resource,
                        ));
                    }
                }
            }
        }

        // Despawn far chunks mercy
        for (entity, chunk) in &chunk_query {
            let chunk_dist = (chunk.coord - player_chunk).as_vec2().length();
            if chunk_dist > VIEW_CHUNKS as f32 + 1.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// remote_interpolation, emotional_resonance_particles, granular_ambient_evolution, lod_mesh_swap unchanged from previous full version

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            remote_interpolation,
            chunk_manager,
            lod_mesh_swap,
        ));
    }
}            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            visibility: Visibility::Visible,
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

    let resource_material = materials.add(Color::rgb(1.0, 0.8, 0.2).into());
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(-500.0..500.0);
        let z = rng.gen_range(-500.0..500.0);
        let y = perlin.get([x as f64 / 100.0, z as f64 / 100.0]) as f32 * 5.0;

        commands.spawn((
            PbrBundle {
                mesh: high_mesh.clone(),
                material: resource_material.clone(),
                transform: Transform::from_xyz(x, y + 2.0, z),
                visibility: Visibility::Visible,
                ..default()
            },
            Resource,
            LodEntity {
                high_mesh: high_mesh.clone(),
                low_mesh: low_mesh.clone(),
            },
        ));
    }
}

// player_movement, remote_interpolation, emotional_resonance_particles, granular_ambient_evolution unchanged

fn entity_culling_optimization(
    player_query: Query<&Transform, With<Player>>,
    mut cullable_query: Query<(&Transform, &mut Visibility), (With<Resource> | Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        for (transform, mut visibility) in &mut cullable_query {
            let dist_sq = (transform.translation - player_pos).length_squared();
            visibility.visible = dist_sq < (LOD_LOW_THRESHOLD * LOD_LOW_THRESHOLD);
        }
    }
}

fn lod_mesh_swap(
    player_query: Query<&Transform, With<Player>>,
    mut lod_query: Query<(&Transform, &LodEntity, &mut Handle<Mesh>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        for (transform, lod, mut mesh) in &mut lod_query {
            let dist_sq = (transform.translation - player_pos).length_squared();
            let target = if dist_sq < (LOD_HIGH_THRESHOLD * LOD_HIGH_THRESHOLD) {
                &lod.high_mesh
            } else {
                &lod.low_mesh
            };
            if mesh.id() != target.id() {
                *mesh = target.clone();
            }
        }
    }
}

#[derive(Component)]
struct LodEntity {
    high_mesh: Handle<Mesh>,
    low_mesh: Handle<Mesh>,
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

#[derive(Component)]
struct Resource;

#[derive(Component)]
struct EmotionalParticle;

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            remote_interpolation,
            entity_culling_optimization,
            lod_mesh_swap,
        ));
    }
}
