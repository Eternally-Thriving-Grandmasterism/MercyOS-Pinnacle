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
use crate::voice::VoicePlugin;

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
    .add_plugins(VoicePlugin)
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
            player_farming_mechanics,
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            weather_system,
            creature_behavior_cycle,
            natural_selection_system,
            creature_hunger_system,
            creature_eat_system,
            crop_growth_system,
            food_respawn_system,
            creature_evolution_system,
            genetic_drift_system,
            player_breeding_mechanics,
            material_attenuation_system,
            chunk_manager,
        ))
        .run();
}

fn material_attenuation_system(
    rapier_context: Res<RapierContext>,
    player_query: Query<&Transform, With<Player>>,
    mut sound_instances: Query<&mut AudioInstance>,
    sound_sources: Query<&SoundSource>,
) {
    if let Ok(listener_transform) = player_query.get_single() {
        let listener_pos = listener_transform.translation;

        for (source, mut instance) in sound_sources.iter().zip(sound_instances.iter_mut()) {
            let direction = source.position - listener_pos;
            let distance = direction.length();
            if distance > 0.1 {
                let ray = Ray::new(Point::from(listener_pos), direction.normalize());

                if let Some((handle, toi)) = rapier_context.cast_ray(ray.origin, ray.dir, distance, true, QueryFilter::default()) {
                    // Get hit voxel material mercy (stub — future chunk lookup)
                    let attenuation = match toi {  // Placeholder material based on distance/toi
                        _ if toi < distance * 0.3 => 0.2,  // Stone-like
                        _ if toi < distance * 0.6 => 0.5,  // Dirt-like
                        _ => 0.8,  // Grass-like open
                    };
                    instance.set_volume(attenuation);
                } else {
                    instance.set_volume(1.0);
                }
            }
        }
    }
}

// Rest of file unchanged from previous full version (add SoundSource component on audio spawns)

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
            crop_growth_system,
            food_respawn_system,
            creature_evolution_system,
            genetic_drift_system,
            player_breeding_mechanics,
            player_farming_mechanics,
            player_inventory_ui,
            material_attenuation_system,
            chunk_manager,
        ));
    }
}
