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
struct Player {
    tamed_creatures: Vec<Entity>,
}

#[derive(Component)]
struct Creature {
    creature_type: CreatureType,
    state: CreatureState,
    wander_timer: f32,
    age: f32,
    health: f32,
    dna: CreatureDNA,
    tamed: bool,
    owner: Option<Entity>,
}

#[derive(Clone, Copy)]
struct CreatureDNA {
    speed: f32,
    size: f32,
    camouflage: f32,
    aggression: f32,
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
    Follow,
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
            player_breeding_mechanics,
            chunk_manager,
        ))
        .run();
}

// setup, advance_time, day_night_cycle, weather_system, get_season, get_biome unchanged from previous

fn player_breeding_mechanics(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(Entity, &Transform, &mut Player)>,
    creature_query: Query<(Entity, &Transform, &mut Creature)>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_transform, mut player)) = player_query.get_single_mut() {
        // Tame nearby creature on key press (E mercy)
        if keyboard_input.just_pressed(KeyCode::E) {
            for (creature_entity, creature_transform, mut creature) in &creature_query {
                let dist = (player_transform.translation - creature_transform.translation).length();
                if dist < 5.0 && !creature.tamed {
                    creature.tamed = true;
                    creature.owner = Some(player_entity);
                    creature.state = CreatureState::Follow;
                    player.tamed_creatures.push(creature_entity);
                }
            }
        }

        // Breeding: select two tamed creatures near player, spawn offspring mercy
        if keyboard_input.just_pressed(KeyCode::B) {
            let mut candidates = Vec::new();
            for &tamed_entity in &player.tamed_creatures {
                if let Ok((_, tamed_transform, tamed_creature)) = creature_query.get(tamed_entity) {
                    let dist = (player_transform.translation - tamed_transform.translation).length();
                    if dist < 10.0 {
                        candidates.push((tamed_entity, tamed_creature.dna));
                    }
                }
            }

            if candidates.len() >= 2 {
                let (parent1_entity, parent1_dna) = candidates[0];
                let (parent2_entity, parent2_dna) = candidates[1];

                let child_dna = CreatureDNA {
                    speed: (parent1_dna.speed + parent2_dna.speed) / 2.0 + rand::thread_rng().gen_range(-1.0..1.0),
                    size: (parent1_dna.size + parent2_dna.size) / 2.0 + rand::thread_rng().gen_range(-0.2..0.2),
                    camouflage: (parent1_dna.camouflage + parent2_dna.camouflage) / 2.0 + rand::thread_rng().gen_range(-0.1..0.1),
                    aggression: (parent1_dna.aggression + parent2_dna.aggression) / 2.0 + rand::thread_rng().gen_range(-0.1..0.1),
                };

                let spawn_pos = player_transform.translation + Vec3::new(rand::thread_rng().gen_range(-5.0..5.0), 2.0, rand::thread_rng().gen_range(-5.0..5.0));

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: child_dna.size })),
                        material: materials.add(Color::rgb(child_dna.camouflage, 0.5, 1.0 - child_dna.camouflage).into()),
                        transform: Transform::from_translation(spawn_pos),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    Creature {
                        creature_type: CreatureType::Deer,  // Simplified
                        state: CreatureState::Follow,
                        wander_timer: 5.0,
                        age: 0.0,
                        health: 1.0,
                        dna: child_dna,
                        tamed: true,
                        owner: Some(player_entity),
                    },
                    Velocity(Vec3::ZERO),
                ));

                player.tamed_creatures.push(/* new entity */);
            }
        }
    }
}

// creature_behavior_cycle updated to handle Follow state (simple towards owner)

// chunk_manager updated to spawn initial creatures with DNA variation

// Other systems unchanged

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
            player_breeding_mechanics,
            chunk_manager,
        ));
    }
}
