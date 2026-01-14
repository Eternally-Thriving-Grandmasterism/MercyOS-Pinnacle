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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Resource)]
struct WorldTime {
    pub day: f32,  // 0.0 to 365.0
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
    .add_plugins(MultiplayerReplicationPlugin)
    .insert_resource(WorldTime { day: 0.0 });

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
            advance_time,
            chunk_manager,
        ))
        .run();
}

fn advance_time(mut time: ResMut<WorldTime>, real_time: Res<Time>) {
    time.day += real_time.delta_seconds() * 0.1;  // Adjustable day speed mercy
    if time.day >= 365.0 {
        time.day -= 365.0;
    }
}

fn get_season(day: f32) -> Season {
    let normalized = day / 365.0;
    if normalized < 0.25 {
        Season::Spring
    } else if normalized < 0.5 {
        Season::Summer
    } else if normalized < 0.75 {
        Season::Autumn
    } else {
        Season::Winter
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
    world_time: Res<WorldTime>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let player_chunk = IVec2::new(
            (player_pos.x / CHUNK_SIZE as f32).floor() as i32,
            (player_pos.z / CHUNK_SIZE as f32).floor() as i32,
        );

        let season = get_season(world_time.day);

        for dx in -VIEW_CHUNKS..=VIEW_CHUNKS {
            for dz in -VIEW_CHUNKS..=VIEW_CHUNKS {
                let chunk_coord = player_chunk + IVec2::new(dx, dz);
                let chunk_exists = chunk_query.iter().any(|(_, chunk)| chunk.coord == chunk_coord);

                if !chunk_exists {
                    let seed = ((chunk_coord.x as u64) << 32) | chunk_coord.y as u64;
                    let density_noise = Perlin::new(seed as u32);
                    let temp_noise = Perlin::new((seed ^ 0x1234) as u32);
                    let humid_noise = Perlin::new((seed ^ 0x5678) as u32);

                    let chunk_center_x = chunk_coord.x as f32 * CHUNK_SIZE as f32 + CHUNK_SIZE as f32 * 0.5;
                    let chunk_center_z = chunk_coord.y as f32 * CHUNK_SIZE as f32 + CHUNK_SIZE as f32 * 0.5;

                    let temperature = (temp_noise.get([chunk_center_x as f64 / 200.0, chunk_center_z as f64 / 200.0]) as f32 + 1.0) * 0.5;
                    let humidity = (humid_noise.get([chunk_center_x as f64 / 200.0, chunk_center_z as f64 / 200.0]) as f32 + 1.0) * 0.5;

                    let biome = get_biome(temperature, humidity);

                    // Seasonal tint modulation mercy
                    let (base_color, season_tint) = match (biome, season) {
                        (Biome::Forest, Season::Autumn) => (Color::rgb(0.1, 0.6, 0.1), Color::rgb(0.8, 0.4, 0.1)),
                        (Biome::Forest, Season::Winter) => (Color::rgb(0.1, 0.6, 0.1), Color::rgb(0.9, 0.9, 0.9)),
                        (Biome::Tundra, Season::Winter) => (Color::rgb(0.8, 0.9, 0.9), Color::rgb(1.0, 1.0, 1.0)),
                        (Biome::Plains, Season::Autumn) => (Color::rgb(0.4, 0.7, 0.3), Color::rgb(0.8, 0.5, 0.1)),
                        _ => (Color::rgb(0.4, 0.7, 0.3), Color::rgb(1.0, 1.0, 1.0)),
                    };

                    let final_color = base_color.lerp(season_tint, match season {
                        Season::Spring => 0.3,
                        Season::Summer => 0.0,
                        Season::Autumn => 0.7,
                        Season::Winter => 0.9,
                    });

                    let grass_mat = materials.add(final_color.into());

                    // Voxel generation unchanged...

                    // Greedy meshing + seasonal material
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                    // Full greedy stubbed

                    let chunk_mesh = meshes.add(mesh);

                    commands.spawn((
                        PbrBundle {
                            mesh: chunk_mesh,
                            material: grass_mat,
                            transform: Transform::from_xyz(chunk_coord.x as f32 * CHUNK_SIZE as f32, 0.0, chunk_coord.y as f32 * CHUNK_SIZE as f32),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        Chunk { coord: chunk_coord },
                        Collider::trimesh_from_mesh(&chunk_mesh).unwrap(),
                        RigidBody::Fixed,
                    ));

                    // Seasonal vegetation modulation
                    let veg_density = match season {
                        Season::Winter => 0.3,
                        Season::Autumn => 0.6,
                        Season::Spring => 1.2,
                        Season::Summer => 1.0,
                    };

                    // Vegetation spawning with seasonal density/color mercy
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
            advance_time,
            chunk_manager,
        ));
    }
}
