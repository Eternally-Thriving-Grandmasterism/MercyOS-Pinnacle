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
struct LeftUpperLeg;

#[derive(Component)]
struct LeftLowerLeg;

#[derive(Component)]
struct RightUpperLeg;

#[derive(Component)]
struct RightLowerLeg;

#[derive(Component)]
struct LeftFootTarget;

#[derive(Component)]
struct RightFootTarget;

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
            multi_chain_ik_system,
            leg_balance_simulation,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    xr_session: Option<Res<XrSession>>,
) {
    // ... unchanged setup with full body chains mercy

    // Legs with foot targets mercy (as before)
}

fn leg_balance_simulation(
    player_query: Query<&Transform, With<Player>>,
    mut spine_query: Query<&mut Transform, Or<(With<SpineLower>, With<SpineMid>, With<SpineUpper>)>>,
    foot_target_query: Query<&Transform, Or<(With<LeftFootTarget>, With<RightFootTarget>)>>,
) {
    let player_transform = player_query.single();

    // Approximate center of mass mercy — torso height
    let com = player_transform.translation + Vec3::new(0.0, 0.5, 0.0);

    let left_foot = foot_target_query.iter().find(|t| t.translation.x < player_transform.translation.x).unwrap_or(&Transform::default());
    let right_foot = foot_target_query.iter().find(|t| t.translation.x >= player_transform.translation.x).unwrap_or(&Transform::default());

    let support_center = (left_foot.translation + right_foot.translation) / 2.0;
    let com_projection = Vec3::new(com.x, 0.0, com.z);
    let offset = support_center - com_projection;

    // Apply spine tilt to correct balance mercy
    let tilt_strength = 0.5;
    let tilt_angle = offset.length() * tilt_strength;
    let tilt_axis = Vec3::Y.cross(offset.normalize_or_zero());

    for mut spine in &mut spine_query {
        spine.rotation = Quat::from_axis_angle(tilt_axis.normalize_or_zero(), tilt_angle) * spine.rotation;
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
            leg_balance_simulation,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}struct Creature {
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
struct LeftUpperLeg;

#[derive(Component)]
struct LeftLowerLeg;

#[derive(Component)]
struct RightUpperLeg;

#[derive(Component)]
struct RightLowerLeg;

#[derive(Component)]
struct LeftFootTarget;

#[derive(Component)]
struct RightFootTarget;

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
            multi_chain_ik_system,
            foot_placement_system,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    xr_session: Option<Res<XrSession>>,
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

    let player_body = commands.spawn((
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
    )).id();

    // Full multi-chain body avatar mercy — legs with foot targets for placement
    let leg_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 0.15, height: 1.0, resolution: 16 }));
    let foot_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.4 }));

    let skin_material = materials.add(Color::rgb(0.9, 0.7, 0.6).into());

    // Left leg chain mercy
    let left_upper_leg = commands.spawn((
        PbrBundle {
            mesh: leg_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(-0.2, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftUpperLeg,
        PlayerBodyPart,
    )).id();

    let left_lower_leg = commands.spawn((
        PbrBundle {
            mesh: leg_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftLowerLeg,
        PlayerBodyPart,
    )).id();

    let left_foot_target = commands.spawn((
        PbrBundle {
            mesh: foot_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftFootTarget,
    )).id();

    commands.entity(left_upper_leg).push_children(&[left_lower_leg]);
    commands.entity(left_lower_leg).push_children(&[left_foot_target]);
    commands.entity(player_body).push_children(&[left_upper_leg]);

    // Right leg symmetric mercy
    let right_upper_leg = commands.spawn((
        PbrBundle {
            mesh: leg_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.2, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightUpperLeg,
        PlayerBodyPart,
    )).id();

    let right_lower_leg = commands.spawn((
        PbrBundle {
            mesh: leg_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightLowerLeg,
        PlayerBodyPart,
    )).id();

    let right_foot_target = commands.spawn((
        PbrBundle {
            mesh: foot_mesh,
            material: skin_material,
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightFootTarget,
    )).id();

    commands.entity(right_upper_leg).push_children(&[right_lower_leg]);
    commands.entity(right_lower_leg).push_children(&[right_foot_target]);
    commands.entity(player_body).push_children(&[right_upper_leg]);

    // Arms, spine, head mercy (as before)

    // XR session override mercy
    if let Some(session) = xr_session {
        // Future: bind head/hand/foot poses
    }
}

fn foot_placement_system(
    rapier_context: Res<RapierContext>,
    player_query: Query<&Transform, With<Player>>,
    mut foot_target_query: Query<&mut Transform, Or<(With<LeftFootTarget>, With<RightFootTarget>)>>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation;

    // Left foot placement mercy
    if let Ok(mut left_foot) = foot_target_query.get_single_mut().ok() {
        let hip = player_pos + Vec3::new(-0.2, -0.4, 0.0);
        let ray = Ray::new(hip.into(), Vec3::NEG_Y.into());

        if let Some((_, toi)) = rapier_context.cast_ray(ray.origin, ray.dir, 2.0, true, QueryFilter::default()) {
            let ground_hit = hip + Vec3::NEG_Y * toi;
            left_foot.translation = ground_hit + Vec3::new(0.0, 0.2, 0.0);  // Foot offset mercy
        }
    }

    // Right foot symmetric mercy
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
            foot_placement_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
