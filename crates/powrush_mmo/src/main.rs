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
const DAY_LENGTH_SECONDS: f32 = 120.0;  // 2-minute day cycle mercy (adjustable)

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
    pub time_of_day: f32,  // 0.0 to 1.0 (0.0 = midnight)
    pub day: f32,          // 0.0 to 365.0
}

#[derive(Resource)]
struct WeatherManager {
    pub current: Weather,
    pub intensity: f32,
    pub duration_timer: f32,
    pub next_change: f32,
}

#[derive(Component)]
struct SunLight;

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

fn day_night_cycle(
    world_time: Res<WorldTime>,
    mut sun_query: Query<&mut Transform, With<SunLight>>,
    mut light_query: Query<&mut DirectionalLight>,
    mut sky_mat: ResMut<Assets<StandardMaterial>>,  // Placeholder for skybox tint
) {
    let time = world_time.time_of_day;
    let sun_angle = time * std::f32::consts::PI * 2.0 - std::f32::consts::PI / 2.0;

    if let Ok(mut sun_transform) = sun_query.get_single_mut() {
        sun_transform.rotation = Quat::from_rotation_y(sun_angle);
    }

    if let Ok(mut light) = light_query.get_single_mut() {
        let intensity = (sun_angle.cos() * 0.5 + 0.5).max(0.05);  // Night low light mercy
        light.illuminance = intensity * 100000.0;

        // Color temperature shift
        let color_temp = if time < 0.25 || time > 0.75 {
            Color::rgb(0.3, 0.3, 0.6)  // Night blue
        } else if time < 0.3 || time > 0.7 {
            Color::rgb(1.0, 0.6, 0.3)  // Sunrise/sunset orange
        } else {
            Color::rgb(1.0, 0.95, 0.9)  // Day white
        };
        light.color = color_temp;
    }

    // Star visibility, sky tint — placeholder mercy
}

fn weather_system(/* unchanged from previous */) { /* same */ }

fn get_season(day: f32) -> Season {
    let normalized = day / 365.0;
    if normalized < 0.25 { Season::Spring }
    else if normalized < 0.5 { Season::Summer }
    else if normalized < 0.75 { Season::Autumn }
    else { Season::Winter }
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

fn chunk_manager(/* unchanged from previous full version */) { /* same */ }

// player_movement, emotional_resonance_particles, granular_ambient_evolution unchanged

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            weather_system,
            chunk_manager,
        ));
    }
}
