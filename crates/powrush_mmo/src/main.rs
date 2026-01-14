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
use bevy_egui::{EguiContexts, EguiPlugin};
use egui::{Painter, Pos2, Stroke, Color32};
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
    show_inventory: bool,
    selected_creature: Option<Entity>,
}

#[derive(Component)]
struct Creature {
    creature_type: CreatureType,
    state: CreatureState,
    wander_timer: f32,
    age: f32,
    health: f32,
    hunger: f32,  // 0.0 full → 1.0 starving
    dna: CreatureDNA,
    tamed: bool,
    owner: Option<Entity>,
    parent1: Option<u64>,
    parent2: Option<u64>,
    generation: u32,
    last_drift_day: f32,
}

#[derive(Clone, Copy)]
struct CreatureDNA {
    speed: f32,
    size: f32,
    camouflage: f32,
    aggression: f32,
    metabolism: f32,  // Hunger rate multiplier
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
    Eat,
    Dead,
}

#[derive(Component)]
struct FoodResource {
    nutrition: f32,
    respawn_timer: f32,
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
    .add_plugins(EguiPlugin)
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
            player_inventory_ui,
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            weather_system,
            creature_behavior_cycle,
            creature_hunger_system,
            creature_eat_system,
            food_respawn_system,
            natural_selection_system,
            creature_evolution_system,
            genetic_drift_system,
            player_breeding_mechanics,
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
        Player {
            tamed_creatures: Vec::new(),
            show_inventory: false,
            selected_creature: None,
        },
        Predicted,
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        PositionHistory { buffer: VecDeque::new() },
    ));
}

fn creature_hunger_system(
    mut query: Query<&mut Creature>,
    time: Res<Time>,
) {
    for mut creature in &mut query {
        let hunger_rate = 0.0005 * creature.dna.metabolism;
        creature.hunger += hunger_rate * time.delta_seconds();
        creature.hunger = creature.hunger.clamp(0.0, 1.0);

        if creature.hunger > 0.8 {
            creature.state = CreatureState::Eat;
        }
    }
}

fn creature_eat_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
    food_query: Query<(Entity, &FoodResource, &Transform)>,
    time: Res<Time>,
) {
    for (creature_entity, mut creature, creature_transform) in &mut creature_query {
        if creature.state == CreatureState::Eat {
            for (food_entity, food, food_transform) in &food_query {
                let dist = (creature_transform.translation - food_transform.translation).length();
                if dist < 3.0 {
                    creature.hunger -= food.nutrition;
                    creature.hunger = creature.hunger.max(0.0);
                    creature.health += food.nutrition * 0.5;
                    creature.health = creature.health.min(1.0);

                    commands.entity(food_entity).despawn();
                    break;
                }
            }

            if creature.hunger < 0.3 {
                creature.state = CreatureState::Wander;
            }
        }
    }
}

fn food_respawn_system(
    mut commands: Commands,
    mut food_query: Query<&mut FoodResource>,
    chunk_query: Query<&Chunk>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut food in &mut food_query {
        food.respawn_timer -= time.delta_seconds();
        if food.respawn_timer <= 0.0 {
            // Respawn logic per biome in chunk_manager mercy
            food.respawn_timer = rand::thread_rng().gen_range(30.0..120.0);
        }
    }

    // New food spawning in chunk_manager
}

// chunk_manager — add food spawning per biome
fn chunk_manager(/* ... */) {
    // In chunk spawn loop
    let food_density = match biome {
        Biome::Forest => 0.1,
        Biome::Plains => 0.08,
        Biome::Desert => 0.02,
        Biome::Tundra => 0.03,
        Biome::Ocean => 0.05,
    };

    let food_mesh = meshes.add(Mesh::from(shape::UVSphere::default()));
    let food_material = materials.add(Color::rgb(0.8, 0.2, 0.2).into());

    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..(food_density * (CHUNK_SIZE * CHUNK_SIZE) as f32) as usize {
        let local_x = rng.gen_range(0.0..CHUNK_SIZE as f32);
        let local_z = rng.gen_range(0.0..CHUNK_SIZE as f32);
        let height = /* from voxel */;

        commands.spawn((
            PbrBundle {
                mesh: food_mesh.clone(),
                material: food_material.clone(),
                transform: Transform::from_xyz(chunk_world_x + local_x, height + 0.5, chunk_world_z + local_z),
                visibility: Visibility::Visible,
                ..default()
            },
            FoodResource {
                nutrition: 0.3,
                respawn_timer: 0.0,
            },
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
            natural_selection_system,
            creature_hunger_system,
            creature_eat_system,
            food_respawn_system,
            creature_evolution_system,
            genetic_drift_system,
            player_breeding_mechanics,
            player_inventory_ui,
            chunk_manager,
        ));
    }
}
