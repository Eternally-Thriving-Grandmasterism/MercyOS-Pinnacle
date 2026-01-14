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
use crate::hrtf_loader::{load_hrtf_sofa, get_hrir_for_direction, apply_hrtf_convolution};
use crate::ambisonics::{setup_ambisonics, ambisonics_encode_system, ambisonics_decode_system};

const CHUNK_SIZE: u32 = 32;
const VIEW_CHUNKS: i32 = 5;
const DAY_LENGTH_SECONDS: f32 = 120.0;
const MAX_RAY_BOUNCES: usize = 5;
const SPEED_OF_SOUND: f32 = 343.0;

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
    samples: Vec<f32>,  // Current frame mercy
}

#[derive(Component)]
struct PlayerHead;

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
            acoustic_raytracing_system,
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

    // Full VR body avatar mercy — visible limbs + IK targets
    let arm_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 0.1, height: 0.8, resolution: 16 }));
    let hand_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.2 }));

    let skin_material = materials.add(Color::rgb(0.9, 0.7, 0.6).into());

    // Left arm + hand target
    let left_arm = commands.spawn((
        PbrBundle {
            mesh: arm_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(-0.3, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftArm,
        PlayerBodyPart,
    )).id();

    commands.spawn((
        PbrBundle {
            mesh: hand_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftHandTarget,
    )).set_parent(left_arm);

    // Right arm + hand target
    let right_arm = commands.spawn((
        PbrBundle {
            mesh: arm_mesh,
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.3, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
            visibility: Visibility::Visible,
            ..default()
        },
        RightArm,
        PlayerBodyPart,
    )).id();

    commands.spawn((
        PbrBundle {
            mesh: hand_mesh,
            material: skin_material,
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightHandTarget,
    )).set_parent(right_arm);

    // Legs unchanged...

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

fn acoustic_raytracing_system(
    rapier_context: Res<RapierContext>,
    listener_query: Query<&Transform, With<PlayerHead>>,
    sources: Query<&SoundSource>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Ok(listener_transform) = listener_query.get_single() {
        let listener_pos = listener_transform.translation;

        for source in &sources {
            let mut current_pos = source.position;
            let mut energy = 1.0;
            let mut delay = 0.0;

            for bounce in 0..MAX_RAY_BOUNCES {
                let direction = listener_pos - current_pos;
                let distance = direction.length();
                if distance < 1.0 {
                    break;
                }

                let ray = Ray::new(current_pos.into(), direction.normalize().into());

                if let Some((_, toi)) = rapier_context.cast_ray(ray.origin, ray.dir, distance, true, QueryFilter::default()) {
                    let hit_point = ray.origin + ray.dir * toi;
                    current_pos = hit_point.into();

                    delay += toi / SPEED_OF_SOUND;
                    energy *= 0.6;  // Reflection loss mercy

                    if energy < 0.05 {
                        break;
                    }
                } else {
                    delay += distance / SPEED_OF_SOUND;
                    break;
                }
            }

            // Spawn delayed echo mercy
            let echo_samples = vec![0.0; 48000];  // Placeholder — future source samples
            let echo = StaticSoundData::from_samples(echo_samples, 48000, StaticSoundSettings::default());

            audio.play(echo)
                .with_volume(energy)
                .with_playback_rate(1.0)  // Doppler separate mercy
                .delay(delay)
                .spatial(true)
                .with_position(source.position)
                .handle();
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
            hand_ik_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            acoustic_raytracing_system,
            chunk_manager,
        ));
    }
}
