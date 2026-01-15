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
const MUTATION_RATE: f32 = 0.1;  // Chance per trait mercy eternal
const MUTATION_STRENGTH: f32 = 0.2;  // Max deviation mercy

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
pub struct CreatureDNA {
    pub speed: f32,
    pub size: f32,
    pub camouflage: f32,
    pub aggression: f32,
    pub metabolism: f32,
    pub lifespan: f32,
    pub fertility: f32,
    pub cold_resistance: f32,
    pub heat_resistance: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
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

#[derive(Component)]
struct SoundSource {
    position: Vec3,
}

#[derive(Component)]
struct PlayerHead;

#[derive(Component)]
struct PlayerBodyPart;

#[derive(Component)]
struct LeftUpperArm;

#[derive(Component)]
struct LeftForearm;

#[derive(Component)]
struct RightUpperArm;

#[derive(Component)]
struct RightForearm;

#[derive(Component)]
struct LeftHandTarget;

#[derive(Component)]
struct RightHandTarget;

#[derive(Resource)]
struct HrtfResource {
    pub data: HrtfData,
}

struct HrtfData {
    sofa: SofaFile,
    sample_rate: u32,
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
    })
    .add_startup_system(load_hrtf_system)
    .add_startup_system(setup_ambisonics);

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
            dynamic_head_tracking,
            player_inventory_ui,
            player_farming_mechanics,
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            creature_breeding_mutation_system,
            creature_genetics_system,
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
            hrtf_convolution_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            vr_body_avatar_system,
            chunk_manager,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    xr_session: Option<Res<XrSession>>,
) {
    // ... unchanged setup
}

fn creature_genetics_system(
    mut creature_query: Query<&mut Creature>,
    time: Res<Time>,
) {
    for mut creature in &mut creature_query {
        creature.age += time.delta_seconds();

        if creature.age > creature.dna.lifespan {
            creature.state = CreatureState::Dead;
        }
    }
}

fn creature_breeding_mutation_system(
    mut commands: Commands,
    creature_query: Query<(Entity, &Transform, &Creature)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut breeding_pairs = Vec::new();

    for (entity1, transform1, creature1) in &creature_query {
        if creature1.state != CreatureState::Mate {
            continue;
        }

        for (entity2, transform2, creature2) in &creature_query {
            if entity1 == entity2 || creature2.state != CreatureState::Mate {
                continue;
            }

            if creature1.creature_type == creature2.creature_type {
                let dist = (transform1.translation - transform2.translation).length();
                if dist < 5.0 {
                    breeding_pairs.push((creature1.dna, creature2.dna, transform1.translation));
                    // Mark as bred mercy
                    break;
                }
            }
        }
    }

    for (dna1, dna2, pos) in breeding_pairs {
        let mut offspring_dna = dna1;

        // Crossover mercy — average traits
        offspring_dna.speed = (dna1.speed + dna2.speed) / 2.0;
        offspring_dna.size = (dna1.size + dna2.size) / 2.0;
        // ... all traits mercy

        // Mutation mercy eternal
        let mut rng = rand::thread_rng();
        if rng.gen_bool(MUTATION_RATE as f64) {
            offspring_dna.speed += rng.gen_range(-MUTATION_STRENGTH..MUTATION_STRENGTH);
            offspring_dna.speed = offspring_dna.speed.clamp(5.0, 30.0);
        }
        // Repeat for other traits mercy

        // Spawn offspring mercy
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: offspring_dna.size / 2.0, subdivisions: 4 })),
                material: materials.add(Color::rgb(offspring_dna.color_r, offspring_dna.color_g, offspring_dna.color_b).into()),
                transform: Transform::from_translation(pos + Vec3::new(rng.gen_range(-2.0..2.0), 0.0, rng.gen_range(-2.0..2.0))),
                visibility: Visibility::Visible,
                ..default()
            },
            Creature {
                creature_type: CreatureType::Deer,  // Example mercy
                state: CreatureState::Wander,
                wander_timer: 10.0,
                age: 0.0,
                health: 1.0,
                hunger: 0.5,
                dna: offspring_dna,
                tamed: false,
                owner: None,
                parent1: None,
                parent2: None,
                generation: 1,
                last_drift_day: 0.0,
            },
        ));
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
            creature_breeding_mutation_system,
            creature_genetics_system,
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
            hrtf_convolution_system,
            dynamic_head_tracking,
            vr_body_avatar_system,
            multi_chain_ik_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
