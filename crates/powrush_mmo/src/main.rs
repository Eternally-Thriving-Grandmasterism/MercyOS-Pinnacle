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
const DAY_LENGTH_SECONDS: f32 = 120.0;

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

#[derive(Clone, Copy, PartialEq)]
enum Weather {
    Clear,
    Rain,
    Snow,
    Storm,
    Fog,
}

#[derive(Resource)]
struct WorldTime {
    pub time_of_day: f32,
    pub day: f32,
}

#[derive(Resource)]
struct WeatherManager {
    pub current: Weather,
    pub intensity: f32,
    pub duration_timer: f32,
    pub next_change: f32,
}

#[derive(Component)]
struct Creature {
    creature_type: CreatureType,
    state: CreatureState,
    wander_timer: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CreatureType {
    Deer,    // Forest diurnal
    Wolf,    // Tundra nocturnal
    Bird,    // Plains diurnal
    Fish,    // Ocean always active
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CreatureState {
    Wander,
    Flee,
    Sleep,
    Migrate,
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
    .insert_resource(WorldTime { time_of_day: 0.0, day: 0.0 })
    .insert_resource(WeatherManager {
        current: Weather::Clear,
        intensity: 0.0,
        duration_timer: 0.0,
        next_change: 300.0,
    });

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
            day_night_cycle,
            weather_system,
            creature_behavior_cycle,
            chunk_manager,
        ))
        .run();
}

fn advance_time(mut world_time: ResMut<WorldTime>, real_time: Res<Time>) {
    world_time.time_of_day += real_time.delta_seconds() / DAY_LENGTH_SECONDS;
    if world_time.time_of_day >= 1.0 {
        world_time.time_of_day -= 1.0;
        world_time.day += 1.0;
        if world_time.day >= 365.0 {
            world_time.day -= 365.0;
        }
    }
}

fn day_night_cycle(/* unchanged */) { /* same as previous */ }

fn weather_system(/* unchanged */) { /* same */ }

fn creature_behavior_cycle(
    world_time: Res<WorldTime>,
    weather: Res<WeatherManager>,
    mut query: Query<(&mut Transform, &mut Creature, &mut Velocity)>,
    time: Res<Time>,
) {
    let is_day = world_time.time_of_day > 0.25 && world_time.time_of_day < 0.75;
    let season = get_season(world_time.day);

    for (mut transform, mut creature, mut velocity) in &mut query {
        let should_sleep = match creature.creature_type {
            CreatureType::Deer | CreatureType::Bird => !is_day,
            CreatureType::Wolf => is_day,
            CreatureType::Fish => false,
        };

        let in_storm = weather.current == Weather::Storm;

        creature.state = if in_storm {
            CreatureState::Flee
        } else if should_sleep {
            CreatureState::Sleep
        } else if season == Season::Winter && creature.creature_type != CreatureType::Fish {
            CreatureState::Migrate
        } else {
            CreatureState::Wander
        };

        match creature.state {
            CreatureState::Wander => {
                creature.wander_timer -= time.delta_seconds();
                if creature.wander_timer <= 0.0 {
                    creature.wander_timer = rand::thread_rng().gen_range(2.0..8.0);
                    let direction = Vec3::new(
                        rand::thread_rng().gen_range(-1.0..1.0),
                        0.0,
                        rand::thread_rng().gen_range(-1.0..1.0),
                    ).normalize_or_zero();
                    velocity.0 = direction * 5.0;
                }
                transform.translation += velocity.0 * time.delta_seconds();
            }
            CreatureState::Sleep => velocity.0 = Vec3::ZERO,
            CreatureState::Flee => velocity.0 = velocity.0.normalize_or_zero() * 15.0,
            CreatureState::Migrate => velocity.0 = Vec3::new(0.0, 0.0, -5.0),  // South mercy placeholder
        }
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
    // Unchanged voxel + biome generation...

    // Creature spawning per biome mercy
    let biome = /* from chunk biome calculation */;
    let creature_density = match biome {
        Biome::Forest => 0.05,
        Biome::Tundra => 0.03,
        Biome::Plains => 0.04,
        Biome::Ocean => 0.02,
        Biome::Desert => 0.01,
    };

    let creature_type = match biome {
        Biome::Forest => CreatureType::Deer,
        Biome::Tundra => CreatureType::Wolf,
        Biome::Plains => CreatureType::Bird,
        Biome::Ocean => CreatureType::Fish,
        Biome::Desert => CreatureType::Deer,  // Rare
    };

    // Spawn creatures in chunk with density
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..(creature_density * (CHUNK_SIZE * CHUNK_SIZE) as f32) as usize {
        let local_x = rng.gen_range(0.0..CHUNK_SIZE as f32);
        let local_z = rng.gen_range(0.0..CHUNK_SIZE as f32);
        let height = /* from voxel height */;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
                material: materials.add(Color::rgb(0.6, 0.4, 0.2).into()),
                transform: Transform::from_xyz(chunk_world_x + local_x, height + 1.0, chunk_world_z + local_z),
                visibility: Visibility::Visible,
                ..default()
            },
            Creature {
                creature_type,
                state: CreatureState::Wander,
                wander_timer: rng.gen_range(2.0..8.0),
            },
            Velocity(Vec3::ZERO),
        ));
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
            day_night_cycle,
            weather_system,
            creature_behavior_cycle,
            chunk_manager,
        ));
    }
}
