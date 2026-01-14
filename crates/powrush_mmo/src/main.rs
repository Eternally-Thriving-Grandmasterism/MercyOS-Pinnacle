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
    in_ragdoll: bool,
    recovering: bool,
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
    Ragdoll,
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
    biome: Biome,
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

#[derive(Component)]
struct RagdollRoot;

#[derive(Resource)]
struct PlayerAnimations {
    pub get_up_prone: Handle<AnimationClip>,
    pub get_up_supine: Handle<AnimationClip>,
    pub get_up_side: Handle<AnimationClip>,
}

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
    .add_plugins(XrSessionPlugin)
    .insert_resource(WorldTime { time_of_day: 0.0, day: 0.0 })
    .insert_resource(WeatherManager {
        current: Weather::Clear,
        intensity: 0.0,
        duration_timer: 0.0,
        next_change: 300.0,
    })
    .add_startup_system(load_hrtf_system)
    .add_startup_system(setup_ambisonics)
    .add_startup_system(load_player_animations);

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
            multi_chain_ik_system,
            ragdoll_transition_system,
            get_up_recovery_system,
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
            hrtf_convolution_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            vr_body_avatar_system,
            chunk_manager,
        ))
        .run();
}

fn load_player_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let get_up_prone = asset_server.load("animations/get_up_prone.glb#Animation0");
    let get_up_supine = asset_server.load("animations/get_up_supine.glb#Animation0");
    let get_up_side = asset_server.load("animations/get_up_side.glb#Animation0");

    commands.insert_resource(PlayerAnimations {
        get_up_prone,
        get_up_supine,
        get_up_side,
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    xr_session: Option<Res<XrSession>>,
    animations: Res<PlayerAnimations>,
) {
    // ... unchanged setup

    let player_body = commands.spawn((
        // ... player bundle
        AnimationPlayer::default(),
    )).id();

    // Pre-load all recovery variants mercy
    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animations.get_up_prone.clone()).repeat(false).paused();
    animation_player.play(animations.get_up_supine.clone()).repeat(false).paused();
    animation_player.play(animations.get_up_side.clone()).repeat(false).paused();

    commands.entity(player_body).insert(animation_player);
}

fn get_up_recovery_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Player, &mut AnimationPlayer, &Transform), With<Player>>,
    ragdoll_query: Query<&Transform, With<RagdollRoot>>,
    animations: Res<PlayerAnimations>,
    weather: Res<WeatherManager>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    for (player_entity, mut player, mut animation_player, player_transform) in &mut player_query {
        if player.in_ragdoll && keyboard_input.just_pressed(KeyCode::Space) {
            // Determine fall orientation mercy
            let ragdoll_transform = ragdoll_query.get_single().unwrap_or(player_transform);
            let forward = ragdoll_transform.forward();
            let up = ragdoll_transform.up();

            let variant = if up.y > 0.7 {
                animations.get_up_supine.clone()
            } else if up.y < -0.7 {
                animations.get_up_prone.clone()
            } else {
                animations.get_up_side.clone()
            };

            // Biome + weather synergy mercy
            let mut speed_multiplier = 1.0;
            let mut slip_chance = 0.0;
            let mut particle_color = Color::rgb(0.2, 0.8, 0.2);

            // Find current chunk biome mercy
            if let Ok((chunk, chunk_transform)) = chunk_query.get_single() {
                let player_local = player_transform.translation - chunk_transform.translation;
                if player_local.x.abs() < CHUNK_SIZE as f32 / 2.0 && player_local.z.abs() < CHUNK_SIZE as f32 / 2.0 {
                    match (chunk.biome, weather.current) {
                        (Biome::Plains, Weather::Rain) => {
                            speed_multiplier = 0.6;  // Mud mercy
                            slip_chance = 0.3;
                            particle_color = Color::rgb(0.6, 0.4, 0.2);
                        }
                        (Biome::Forest, Weather::Rain) => {
                            speed_multiplier = 0.7;
                            slip_chance = 0.2;
                            particle_color = Color::rgb(0.1, 0.6, 0.1);
                        }
                        (Biome::Tundra, Weather::Snow) => {
                            speed_multiplier = 0.8;  // Soft snow mercy
                            particle_color = Color::rgb(0.9, 0.9, 1.0);
                        }
                        (Biome::Desert, Weather::Storm) => {
                            speed_multiplier = 0.5;  // Sandstorm mercy
                            slip_chance = 0.4;
                            particle_color = Color::rgb(0.9, 0.8, 0.5);
                        }
                        (_, Weather::Storm) => {
                            speed_multiplier = 0.7;  // Wind sway mercy
                        }
                        _ => speed_multiplier = 1.0,
                    }
                }
            }

            // Slip chance mercy
            if slip_chance > 0.0 && rand::thread_rng().gen_bool(slip_chance) {
                player.in_ragdoll = true;
                // Slip sound + particles mercy
                continue;
            }

            // Despawn ragdoll mercy
            for ragdoll in &ragdoll_query {
                commands.entity(ragdoll.entity()).despawn_recursive();
            }

            // Crossfade with synergy speed + particles mercy
            animation_player.play_with_transition(variant, Duration::from_secs_f32(0.3))
                .set_speed(speed_multiplier)
                .repeat(false);

            player.in_ragdoll = false;
            player.recovering = true;

            // Synergy particles mercy
            // Spawn colored mud/snow/sand sparkles
        }

        if player.recovering && animation_player.is_finished() {
            player.recovering = false;
            // Massive joy burst mercy
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
            hrtf_convolution_system,
            dynamic_head_tracking,
            vr_body_avatar_system,
            multi_chain_ik_system,
            ragdoll_transition_system,
            get_up_recovery_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
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
    Ragdoll,
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
    biome: Biome,  // Biome tag mercy eternal
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

#[derive(Component)]
struct RagdollRoot;

#[derive(Resource)]
struct PlayerAnimations {
    pub get_up_prone: Handle<AnimationClip>,
    pub get_up_supine: Handle<AnimationClip>,
    pub get_up_side: Handle<AnimationClip>,
}

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
    .add_plugins(XrSessionPlugin)
    .insert_resource(WorldTime { time_of_day: 0.0, day: 0.0 })
    .insert_resource(WeatherManager {
        current: Weather::Clear,
        intensity: 0.0,
        duration_timer: 0.0,
        next_change: 300.0,
    })
    .add_startup_system(load_hrtf_system)
    .add_startup_system(setup_ambisonics)
    .add_startup_system(load_player_animations);

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
            multi_chain_ik_system,
            ragdoll_transition_system,
            get_up_recovery_system,
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
            hrtf_convolution_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            vr_body_avatar_system,
            chunk_manager,
        ))
        .run();
}

fn load_player_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let get_up_prone = asset_server.load("animations/get_up_prone.glb#Animation0");
    let get_up_supine = asset_server.load("animations/get_up_supine.glb#Animation0");
    let get_up_side = asset_server.load("animations/get_up_side.glb#Animation0");

    commands.insert_resource(PlayerAnimations {
        get_up_prone,
        get_up_supine,
        get_up_side,
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    xr_session: Option<Res<XrSession>>,
    animations: Res<PlayerAnimations>,
) {
    // ... unchanged setup

    let player_body = commands.spawn((
        // ... player bundle
        AnimationPlayer::default(),
    )).id();

    // Pre-load all recovery variants mercy
    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animations.get_up_prone.clone()).repeat(false).paused();
    animation_player.play(animations.get_up_supine.clone()).repeat(false).paused();
    animation_player.play(animations.get_up_side.clone()).repeat(false).paused();

    commands.entity(player_body).insert(animation_player);
}

fn get_up_recovery_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Player, &mut AnimationPlayer, &Transform), With<Player>>,
    ragdoll_query: Query<&Transform, With<RagdollRoot>>,
    animations: Res<PlayerAnimations>,
    chunk_query: Query<(&Chunk, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    for (player_entity, mut player, mut animation_player, player_transform) in &mut player_query {
        if player.in_ragdoll && keyboard_input.just_pressed(KeyCode::Space) {
            // Determine fall orientation mercy
            let ragdoll_transform = ragdoll_query.get_single().unwrap_or(player_transform);
            let forward = ragdoll_transform.forward();
            let up = ragdoll_transform.up();

            let variant = if up.y > 0.7 {
                animations.get_up_supine.clone()
            } else if up.y < -0.7 {
                animations.get_up_prone.clone()
            } else {
                animations.get_up_side.clone()
            };

            // Biome-specific influence mercy
            let mut speed_multiplier = 1.0;
            let mut slip_chance = 0.0;
            let mut particle_color = Color::rgb(0.2, 0.8, 0.2);  // Default grass mercy

            // Find current chunk biome mercy
            if let Ok((chunk, chunk_transform)) = chunk_query.get_single() {
                let player_local = player_transform.translation - chunk_transform.translation;
                if player_local.x.abs() < CHUNK_SIZE as f32 / 2.0 && player_local.z.abs() < CHUNK_SIZE as f32 / 2.0 {
                    match chunk.biome {
                        Biome::Plains => {
                            speed_multiplier = 1.0;
                            particle_color = Color::rgb(0.8, 0.8, 0.2);  // Dry grass mercy
                        }
                        Biome::Forest => {
                            speed_multiplier = 1.2;  // Soft leaves mercy
                            particle_color = Color::rgb(0.1, 0.6, 0.1);
                        }
                        Biome::Desert => {
                            speed_multiplier = 0.7;  // Sand slow mercy
                            particle_color = Color::rgb(0.9, 0.8, 0.5);
                        }
                        Biome::Tundra => {
                            speed_multiplier = 0.5;
                            slip_chance = 0.4;  // Snow slippery mercy
                            particle_color = Color::rgb(0.9, 0.9, 1.0);
                        }
                        Biome::Ocean => {
                            speed_multiplier = 0.3;  // Water float mercy
                            particle_color = Color::rgb(0.2, 0.6, 0.9);
                        }
                    }
                }
            }

            // Slip chance mercy
            if slip_chance > 0.0 && rand::thread_rng().gen_bool(slip_chance) {
                player.in_ragdoll = true;
                // Slip sound + particles mercy
                continue;
            }

            // Despawn ragdoll mercy
            for ragdoll in &ragdoll_query {
                commands.entity(ragdoll.entity()).despawn_recursive();
            }

            // Crossfade with biome speed + particles mercy
            animation_player.play_with_transition(variant, Duration::from_secs_f32(0.3))
                .set_speed(speed_multiplier)
                .repeat(false);

            player.in_ragdoll = false;
            player.recovering = true;

            // Biome particles mercy
            // Spawn colored leaf/sand/snow/water sparkles
        }

        if player.recovering && animation_player.is_finished() {
            player.recovering = false;
            // Massive joy burst mercy
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
            hrtf_convolution_system,
            dynamic_head_tracking,
            vr_body_avatar_system,
            multi_chain_ik_system,
            ragdoll_transition_system,
            get_up_recovery_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
