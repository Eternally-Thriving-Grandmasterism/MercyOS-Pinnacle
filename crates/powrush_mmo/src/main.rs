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
const CLIMB_SPEED: f32 = 3.0;
const WALL_CHECK_DISTANCE: f32 = 1.0;

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
    climbing: bool,
    wall_normal: Vec3,
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
            wall_climbing_system,
            dynamic_head_tracking,
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
            climbing: false,
            wall_normal: Vec3::ZERO,
        },
        Predicted,
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        PositionHistory { buffer: VecDeque::new() },
    )).id();

    // Full VR body avatar mercy — visible limbs + IK targets (as before)

    // Head separate for VR tracking mercy
    commands.spawn((
        Transform::from_xyz(0.0, 1.8, 0.0),
        GlobalTransform::default(),
        PlayerHead,
    )).set_parent(player_body);

    // XR session override mercy
    if let Some(session) = xr_session {
        // Future: bind head/hand poses
    }
}

fn player_movement(
    mut player_query: Query<(&mut Velocity, &mut Player, &Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((mut velocity, mut player, transform)) = player_query.get_single_mut() {
        if player.climbing {
            // Climbing movement mercy
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::W) {
                direction += player.wall_normal.cross(transform.right());  // Up along wall mercy
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction -= player.wall_normal.cross(transform.right());
            }

            if direction.length_squared() > 0.0 {
                direction = direction.normalize();
            }

            velocity.linvel = direction * CLIMB_SPEED;
        } else {
            // Normal ground movement mercy
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::W) {
                direction += transform.forward();
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction += transform.back();
            }
            if keyboard_input.pressed(KeyCode::A) {
                direction += transform.left();
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction += transform.right();
            }

            direction.y = 0.0;
            if direction.length_squared() > 0.0 {
                direction = direction.normalize();
            }

            velocity.linvel.x = direction.x * 8.0;
            velocity.linvel.z = direction.z * 8.0;
        }
    }
}

fn wall_climbing_system(
    mut player_query: Query<(&mut Player, &Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
) {
    for (mut player, transform, mut velocity) in &mut player_query {
        let forward = transform.forward();
        let ray = Ray::new(transform.translation.into(), forward.into());

        if let Some((_, toi, normal)) = rapier_context.cast_ray_and_get_normal(ray.origin, ray.dir, WALL_CHECK_DISTANCE, true, QueryFilter::default()) {
            let wall_normal = Vec3::from(normal);

            // Climbable if steep mercy
            if wall_normal.y.abs() < 0.7 {
                if keyboard_input.pressed(KeyCode::W) {
                    player.climbing = true;
                    player.wall_normal = wall_normal;
                    velocity.linvel.y = 0.0;  // Cancel gravity mercy
                }
            } else {
                player.climbing = false;
            }
        } else {
            player.climbing = false;
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
            player_movement,
            wall_climbing_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}    coyote_timer: f32,
    jump_buffer_timer: f32,
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
            jump_system,
            dynamic_head_tracking,
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
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
        },
        Predicted,
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        PositionHistory { buffer: VecDeque::new() },
    )).id();

    // Full VR body avatar mercy — visible limbs + IK targets (as before)

    // Head separate for VR tracking mercy
    commands.spawn((
        Transform::from_xyz(0.0, 1.8, 0.0),
        GlobalTransform::default(),
        PlayerHead,
    )).set_parent(player_body);

    // XR session override mercy
    if let Some(session) = xr_session {
        // Future: bind head/hand poses
    }
}

fn player_movement(
    mut player_query: Query<(&mut Velocity, &mut Player, &Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, mut player, transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += transform.back();
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += transform.right();
        }

        direction.y = 0.0;
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        velocity.linvel.x = direction.x * 8.0;
        velocity.linvel.z = direction.z * 8.0;
    }
}

fn jump_system(
    mut player_query: Query<(&mut Velocity, &mut Player, &Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (mut velocity, mut player, transform) in &mut player_query {
        let pos = transform.translation;

        // Grounded check mercy — raycast down
        let ray = Ray::new(pos.into(), Vec3::NEG_Y.into());
        let grounded = rapier_context.cast_ray(ray.origin, ray.dir, 1.2, true, QueryFilter::default()).is_some();

        // Coyote time mercy
        if grounded {
            player.coyote_timer = COYOTE_TIME;
        } else if player.coyote_timer > 0.0 {
            player.coyote_timer -= time.delta_seconds();
        }

        // Jump buffer mercy
        if keyboard_input.just_pressed(KeyCode::Space) {
            player.jump_buffer_timer = JUMP_BUFFER_TIME;
        } else if player.jump_buffer_timer > 0.0 {
            player.jump_buffer_timer -= time.delta_seconds();
        }

        // Jump mercy eternal
        if player.jump_buffer_timer > 0.0 && player.coyote_timer > 0.0 {
            velocity.linvel.y = JUMP_IMPULSE;
            player.jump_buffer_timer = 0.0;
            player.coyote_timer = 0.0;

            // Joy particles on jump mercy
            // Spawn upward sparkles
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
            player_movement,
            jump_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}    health: f32,
    hunger: f32,
    dna: CreatureDNA,
    tamed: bool,
    owner: Option<Entity>,
    parent1: Option<u64>,
    parent2: Option<u64>,
    generation: u32,
    last_drift_day: f32,
    perception_radius: f32,
    memory_target: Option<Vec3>,
    memory_timer: f32,
    current_goal: Option<Vec3>,
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
            advanced_creature_ai_system,
            creature_movement_system,
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

fn advanced_creature_ai_system(
    mut creature_query: Query<(&Transform, &mut Creature, &CreatureDNA)>,
    player_query: Query<&Transform, With<Player>>,
    other_creature_query: Query<(&Transform, &Creature), Without<Player>>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    let player_transform = player_query.single();

    for (transform, mut creature, dna) in &mut creature_query {
        let pos = transform.translation;

        // Perception mercy — detect threats/prey/pack
        let mut threats = Vec::new();
        let mut prey = Vec::new();
        let mut pack = Vec::new();

        for (other_transform, other_creature) in &other_creature_query {
            let dist = (pos - other_transform.translation).length();
            if dist < creature.perception_radius {
                match (creature.creature_type, other_creature.creature_type) {
                    (CreatureType::Deer, CreatureType::Wolf) => threats.push(other_transform.translation),
                    (CreatureType::Wolf, CreatureType::Deer) => prey.push(other_transform.translation),
                    _ if creature.creature_type == other_creature.creature_type => pack.push(other_transform.translation),
                    _ => {}
                }
            }
        }

        // Memory mercy
        if creature.memory_timer > 0.0 {
            creature.memory_timer -= time.delta_seconds();
        } else {
            creature.memory_target = None;
        }

        // Goal priority mercy eternal
        let mut goal = None;

        if !threats.is_empty() {
            creature.state = CreatureState::Flee;
            goal = Some(threats[0]);  // Flee from nearest
        } else if !prey.is_empty() && creature.creature_type == CreatureType::Wolf {
            creature.state = CreatureState::Follow;
            goal = Some(prey[0]);
            creature.memory_target = goal;
            creature.memory_timer = 10.0;
        } else if !pack.is_empty() && creature.hunger < 0.5 {
            creature.state = CreatureState::Follow;
            goal = Some(pack.iter().fold(Vec3::ZERO, |a, b| a + *b) / pack.len() as f32);
        } else if creature.hunger > 0.8 {
            creature.state = CreatureState::Eat;
            // Find food mercy — future
        } else {
            creature.state = CreatureState::Wander;
            creature.wander_timer -= time.delta_seconds();
            if creature.wander_timer <= 0.0 {
                creature.wander_timer = 10.0;
            }
        }

        creature.current_goal = goal;
    }
}

fn creature_movement_system(
    mut creature_query: Query<(&mut Velocity, &Transform, &Creature)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (mut velocity, transform, creature) in &mut creature_query {
        let pos = transform.translation;

        if let Some(goal) = creature.current_goal {
            let direction = (goal - pos).normalize_or_zero();

            // Basic pathfinding avoidance mercy
            let ray = Ray::new(pos.into(), direction.into());
            if let Some((_, toi)) = rapier_context.cast_ray(ray.origin, ray.dir, 5.0, true, QueryFilter::default()) {
                if toi < 2.0 {
                    // Blocked — try left/right offset mercy
                    let left = Quat::from_rotation_y(0.5) * direction;
                    let right = Quat::from_rotation_y(-0.5) * direction;

                    let left_ray = Ray::new(pos.into(), left.into());
                    let right_ray = Ray::new(pos.into(), right.into());

                    let left_clear = rapier_context.cast_ray(left_ray.origin, left_ray.dir, 5.0, true, QueryFilter::default()).map_or(true, |(_, t)| t > 3.0);
                    let right_clear = rapier_context.cast_ray(right_ray.origin, right_ray.dir, 5.0, true, QueryFilter::default()).map_or(true, |(_, t)| t > 3.0);

                    let avoid_dir = if left_clear && right_clear {
                        if rand::thread_rng().gen_bool(0.5) { left } else { right }
                    } else if left_clear {
                        left
                    } else if right_clear {
                        right
                    } else {
                        -direction  // Back up mercy
                    };

                    velocity.linvel = avoid_dir * creature.dna.speed;
                } else {
                    velocity.linvel = direction * creature.dna.speed;
                }
            } else {
                velocity.linvel = direction * creature.dna.speed;
            }
        } else if creature.state == CreatureState::Wander {
            // Random wander mercy
            let wander_dir = Vec3::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                0.0,
                rand::thread_rng().gen_range(-1.0..1.0),
            ).normalize_or_zero();

            velocity.linvel = wander_dir * creature.dna.speed * 0.5;
        } else {
            velocity.linvel = Vec3::ZERO;
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
            advanced_creature_ai_system,
            creature_movement_system,
            player_breeding_mechanics,
            player_inventory_ui,
            chunk_manager,
        ));
    }
}
