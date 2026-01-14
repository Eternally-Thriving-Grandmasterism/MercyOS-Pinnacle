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
    age: f32,
    health: f32,
    dna: CreatureDNA,
}

#[derive(Clone, Copy)]
struct CreatureDNA {
    speed: f32,         // 5.0-15.0
    size: f32,          // 0.5-2.0 scale
    camouflage: f32,    // 0.0-1.0 biome fit
    aggression: f32,    // 0.0-1.0
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CreatureType {
    Deer,
    Wolf,
    Bird,
    Fish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CreatureState {
    Wander,
    Flee,
    Sleep,
    Mate,
    Dead,
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
            creature_evolution_system,
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

fn day_night_cycle(/* unchanged */) { /* same */ }

fn weather_system(/* unchanged */) { /* same */ }

fn creature_behavior_cycle(
    world_time: Res<WorldTime>,
    weather: Res<WeatherManager>,
    mut query: Query<(&mut Transform, &mut Creature, &mut Velocity)>,
    time: Res<Time>,
) {
    // Unchanged from previous...
}

fn creature_evolution_system(
    mut commands: Commands,
    world_time: Res<WorldTime>,
    weather: Res<WeatherManager>,
    mut query: Query<(Entity, &mut Creature, &Transform)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let season = get_season(world_time.day);
    let is_day = world_time.time_of_day > 0.25 && world_time.time_of_day < 0.75;

    let mut offspring = Vec::new();

    for (entity, mut creature, transform) in &mut query {
        creature.age += time.delta_seconds();
        creature.health -= time.delta_seconds() * 0.001;  // Natural decay

        // Survival selection mercy
        let biome_fit = creature.dna.camouflage;  // Simplified
        let weather_penalty = if weather.current == Weather::Storm { 0.1 } else { 0.0 };
        creature.health -= weather_penalty * time.delta_seconds();

        if creature.health <= 0.0 || creature.age > 3650.0 {  // 10 years mercy
            commands.entity(entity).despawn();
            continue;
        }

        // Reproduction mercy
        if creature.state == CreatureState::Mate && creature.age > 365.0 {
            for (other_entity, other_creature, other_transform) in &query {
                if entity != other_entity && other_creature.creature_type == creature.creature_type {
                    let dist = (transform.translation - other_transform.translation).length();
                    if dist < 10.0 && rand::thread_rng().gen::<f32>() < 0.01 {
                        let child_dna = CreatureDNA {
                            speed: (creature.dna.speed + other_creature.dna.speed) / 2.0 + rand::thread_rng().gen_range(-1.0..1.0),
                            size: (creature.dna.size + other_creature.dna.size) / 2.0 + rand::thread_rng().gen_range(-0.2..0.2),
                            camouflage: (creature.dna.camouflage + other_creature.dna.camouflage) / 2.0 + rand::thread_rng().gen_range(-0.1..0.1),
                            aggression: (creature.dna.aggression + other_creature.dna.aggression) / 2.0 + rand::thread_rng().gen_range(-0.1..0.1),
                        };

                        offspring.push((transform.translation + Vec3::new(rand::thread_rng().gen_range(-5.0..5.0), 2.0, rand::thread_rng().gen_range(-5.0..5.0)), child_dna, creature.creature_type));
                    }
                }
            }
        }
    }

    // Spawn offspring mercy
    for (pos, dna, ctype) in offspring {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: dna.size })),
                material: materials.add(Color::rgb(dna.camouflage, 0.5, 1.0 - dna.camouflage).into()),
                transform: Transform::from_translation(pos),
                visibility: Visibility::Visible,
                ..default()
            },
            Creature {
                creature_type: ctype,
                state: CreatureState::Wander,
                wander_timer: 5.0,
                age: 0.0,
                health: 1.0,
                dna,
            },
            Velocity(Vec3::ZERO),
        ));
    }
}

fn get_season(day: f32) -> Season {
    let normalized = day / 365.0;
    if normalized < 0.25 { Season::Spring }
    else if normalized < 0.5 { Season::Summer }
    else if normalized < 0.75 { Season::Autumn }
    else { Season::Winter }
}

// chunk_manager, player_movement, emotional_resonance_particles, granular_ambient_evolution unchanged

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
            creature_evolution_system,
            chunk_manager,
        ));
    }
}
