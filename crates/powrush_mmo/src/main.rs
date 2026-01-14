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
use bevy_mod_xr::session::{XrSession, XrSessionPlugin};
use bevy_mod_xr::hands::{XrHand, XrHandBone};
use bevy_mod_xr::spaces::{XrReferenceSpace, XrReferenceSpaceType};
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;
use crate::voice::VoicePlugin;
use crate::hrtf_loader::{load_hrtf_sofa, get_hrir_for_direction, apply_hrtf_convolution};
use crate::ambisonics::{setup_ambisonics, ambisonics_encode_system, ambisonics_decode_system};
use crate::hand_ik::{fabrik_constrained, trik_two_bone};

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
    hunger: f32,
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
    metabolism: f32,
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
struct Crop {
    crop_type: CropType,
    growth_stage: u8,
    growth_timer: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CropType {
    Wheat,
    Berries,
    Roots,
}

#[derive(Component)]
struct Chunk {
    coord: IVec2,
    voxels: Box<[u8; ChunkShape::SIZE as usize]>,
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
            seasonal_biome_weather_system,
            creature_behavior_cycle,
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

fn advance_time(
    mut time: ResMut<WorldTime>,
    game_time: Res<Time>,
) {
    time.time_of_day += game_time.delta_seconds() / DAY_LENGTH_SECONDS * 24.0;
    if time.time_of_day >= 24.0 {
        time.time_of_day -= 24.0;
        time.day += 1.0;
    }
}

fn seasonal_biome_weather_system(
    mut weather: ResMut<WeatherManager>,
    world_time: Res<WorldTime>,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    let season = match (world_time.day % 365.0 / 365.0 * 4.0) as u32 {
        0 => Season::Spring,
        1 => Season::Summer,
        2 => Season::Autumn,
        _ => Season::Winter,
    };

    // Find player biome mercy
    let player_pos = player_query.single().translation;
    let mut current_biome = Biome::Plains;  // Default mercy

    for (chunk, chunk_transform) in &chunk_query {
        let local = player_pos - chunk_transform.translation;
        if local.x.abs() < CHUNK_SIZE as f32 / 2.0 && local.z.abs() < CHUNK_SIZE as f32 / 2.0 {
            current_biome = chunk.biome;
            break;
        }
    }

    // Seasonal biome weather probabilities mercy eternal
    let weather_roll = rand::thread_rng().gen_range(0.0..1.0);

    let new_weather = match (season, current_biome) {
        (Season::Spring, Biome::Forest | Biome::Plains) => {
            if weather_roll < 0.4 { Weather::Rain } else { Weather::Clear }
        }
        (Season::Summer, Biome::Desert) => Weather::Clear,
        (Season::Summer, _) => {
            if weather_roll < 0.2 { Weather::Storm } else { Weather::Clear }
        }
        (Season::Autumn, _) => {
            if weather_roll < 0.3 { Weather::Fog } else { Weather::Clear }
        }
        (Season::Winter, Biome::Tundra) => {
            if weather_roll < 0.6 { Weather::Snow } else { Weather::Clear }
        }
        (Season::Winter, _) => {
            if weather_roll < 0.3 { Weather::Snow } else { Weather::Clear }
        }
        (_, Biome::Ocean) => {
            if weather_roll < 0.4 { Weather::Rain } else { Weather::Fog }
        }
        _ => Weather::Clear,
    };

    if new_weather != weather.current {
        weather.current = new_weather;
        weather.intensity = rand::thread_rng().gen_range(0.3..1.0);
        weather.duration_timer = rand::thread_rng().gen_range(120.0..600.0);
        weather.next_change = weather.duration_timer;
    } else {
        weather.next_change -= game_time.delta_seconds();
        if weather.next_change <= 0.0 {
            // Change weather mercy
            weather.current = Weather::Clear;  // Placeholder transition mercy
        }
    }
}

// Rest of file unchanged from previous full version

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            seasonal_biome_weather_system,
            creature_behavior_cycle,
            player_breeding_mechanics,
            player_inventory_ui,
            chunk_manager,
        ));
    }
}
