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
    pub day: f32,
}

#[derive(Resource)]
struct WeatherManager {
    pub current: Weather,
    pub intensity: f32,  // 0.0-1.0
    pub duration_timer: f32,
    pub next_change: f32,
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
    .insert_resource(WorldTime { day: 0.0 })
    .insert_resource(WeatherManager {
        current: Weather::Clear,
        intensity: 0.0,
        duration_timer: 0.0,
        next_change: 300.0,  // 5 minutes initial
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
            weather_system,
            chunk_manager,
        ))
        .run();
}

fn advance_time(mut time: ResMut<WorldTime>, real_time: Res<Time>) {
    time.day += real_time.delta_seconds() * 0.1;
    if time.day >= 365.0 {
        time.day -= 365.0;
    }
}

fn weather_system(
    mut weather: ResMut<WeatherManager>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
) {
    weather.duration_timer += time.delta_seconds();
    if weather.duration_timer >= weather.next_change {
        weather.duration_timer = 0.0;
        weather.next_change = rand::thread_rng().gen_range(180.0..600.0);  // 3-10 min cycles

        weather.current = match weather.current {
            Weather::Clear => *[Weather::Rain, Weather::Fog, Weather::Storm].choose(&mut rand::thread_rng()).unwrap(),
            Weather::Rain => *[Weather::Clear, Weather::Storm].choose(&mut rand::thread_rng()).unwrap(),
            Weather::Snow => *[Weather::Clear, Weather::Fog].choose(&mut rand::thread_rng()).unwrap(),
            Weather::Storm => Weather::Rain,
            Weather::Fog => Weather::Clear,
        };

        weather.intensity = rand::thread_rng().gen_range(0.3..1.0);
    }

    // Spawn weather particles/visuals mercy
    match weather.current {
        Weather::Rain => {
            // Rain particles + audio
            // Placeholder — full particle system in future
            let rain_sound = ultimate_fm_synthesis(100.0, weather.intensity * 5.0, 10.0);
            audio.play(rain_sound).looped().with_volume(weather.intensity * 0.4);
        }
        Weather::Snow => {
            // Snow flakes + wind
        }
        Weather::Storm => {
            // Thunder + heavy rain
        }
        Weather::Fog => {
            // Fog volume + tint
        }
        Weather::Clear => {
            // Clear audio fade
        }
    }
}

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

// chunk_manager, player_movement, emotional_resonance_particles, granular_ambient_evolution unchanged from previous full version

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            weather_system,
            chunk_manager,
        ));
    }
}
