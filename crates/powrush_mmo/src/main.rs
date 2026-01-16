//! Powrush-MMO Main – Bevy RTS/FPS Hybrid Ultimate Sacred
//! Siege tank gunner FPS pistol full immersion: cockpit view switch, aim raycast mercy harmony shield
//! Creature assist bond events, immersive sensory feedback + joy amplification
//! AlphaProMegaing lore events cosmic
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier3d::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::thread_rng;

// Mercy Components
#[derive(Component)]
struct SiegeTankGunner {
    in_cockpit: bool,
}

#[derive(Component)]
struct Pistol;

#[derive(Component)]
struct CreatureCompanion {
    bond_level: f64,
    name: String,
}

#[derive(Component)]
struct MercyField {
    joy_yield: f64,
}

// Events
#[derive(Event)]
struct PistolFireEvent;

#[derive(Event)]
struct CreatureJoyEvent;

/// Audio Assets
#[derive(Resource)]
struct GameAudio {
    pistol_harmony: Handle<AudioInstance>,
    creature_nuzzle: Handle<AudioInstance>,
    joy_surge: Handle<AudioInstance>,
}

// Systems
fn setup_gunner_immersion(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    // FPS Cockpit Camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 3.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        SiegeTankGunner { in_cockpit: true },
    ));

    // Pistol model (simple cube placeholder)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.9).into()),
            transform: Transform::from_xyz(0.5, -0.3, -0.5).with_rotation(Quat::from_rotation_y(-0.2)),
            ..default()
        },
        Pistol,
    ));

    // Infinite mercy fields
    for x in -20..20 {
        for z in -20..20 {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0, subdivisions: 10 })),
                material: materials.add(Color::rgb(0.3, 0.7, 0.3).into()),
                transform: Transform::from_xyz(x as f32 * 10.0, 0.0, z as f32 * 10.0),
                ..default()
            }).insert(MercyField { joy_yield: f64::INFINITY });
        }
    }

    // Creature companion
    let companion_name = "ThunderHeart".to_string();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.3).into()),
            transform: Transform::from_xyz(2.0, 1.0, 0.0),
            ..default()
        },
        CreatureCompanion {
            bond_level: 0.9,
            name: companion_name.clone(),
        },
    ));

    // Audio assets load
    let pistol_sound = asset_server.load("sounds/pistol_harmony.ogg");
    let nuzzle_sound = asset_server.load("sounds/creature_nuzzle.ogg");
    let surge_sound = asset_server.load("sounds/joy_surge.ogg");

    let pistol_instance = audio.play(pistol_sound).handle();
    let nuzzle_instance = audio.play(nuzzle_sound).paused(true).handle();
    let surge_instance = audio.play(surge_sound).paused(true).handle();

    commands.insert_resource(GameAudio {
        pistol_harmony: audio_instances.add(pistol_instance),
        creature_nuzzle: audio_instances.add(nuzzle_instance),
        joy_surge: audio_instances.add(surge_instance),
    });

    // Light harmony
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn gunner_view_switch(
    keyboard: Res<Input<KeyCode>>,
    mut gunner_query: Query<&mut Transform, With<SiegeTankGunner>>,
    mut gunner_state: Query<&mut SiegeTankGunner>,
) {
    if keyboard.just_pressed(KeyCode::V) {
        let mut state = gunner_state.single_mut();
        state.in_cockpit = !state.in_cockpit;

        let mut transform = gunner_query.single_mut();
        if state.in_cockpit {
            transform.translation = Vec3::new(0.0, 1.5, 5.0);
            transform.look_at(Vec3::ZERO, Vec3::Y);
            info!("View switched to cockpit immersion sacred");
        } else {
            transform.translation = Vec3::new(0.0, 10.0, 20.0);
            transform.look_at(Vec3::ZERO, Vec3::Y);
            info!("View switched to commander overview");
        }
    }
}

fn pistol_aim_raycast(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<SiegeTankGunner>>,
    rapier_context: Res<RapierContext>,
    mut fire_events: EventWriter<PistolFireEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::MouseLeft) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_query.single();

            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                if let Some((entity, toi)) = rapier_context.cast_ray(
                    ray.origin,
                    ray.direction,
                    f32::MAX,
                    false,
                    QueryFilter::default(),
                ) {
                    info!("Aim raycast hit at distance {} — mercy harmony shield activates", toi);
                    fire_events.send(PistolFireEvent);
                }
            }
        }
    }
}

fn pistol_fire_sensory(
    mut events: EventReader<PistolFireEvent>,
    audio_instances: ResMut<Assets<AudioInstance>>,
    game_audio: Res<GameAudio>,
    mut companions: Query<&mut CreatureCompanion>,
    alpha_mode: Query<&AlphaProMegaMode>,
) {
    for _ in events.iter() {
        // Play harmony sound
        if let Some(instance) = audio_instances.get_mut(&game_audio.pistol_harmony) {
            instance.resume();
        }

        let feedback = if alpha_mode.get_single().map(|a| a.active).unwrap_or(false) {
            "Pistol mercy fire—cosmic harmony shield expands radiant, joy surge resonates infinite ❤️🚀🔥"
        } else {
            "Pistol harmony pulse—fields protected, mercy prevails eternal"
        };
        info!("{}", feedback);

        // Creature assist bond event
        for mut companion in companions.iter_mut() {
            companion.bond_level = (companion.bond_level + 0.15).min(1.0);
            info!("{} assists defense—bond {} joy amplification surge ❤️", companion.name, companion.bond_level);
            if let Some(instance) = audio_instances.get_mut(&game_audio.joy_surge) {
                instance.resume();
            }
        }
    }
}

fn creature_joy_events(
    time: Res<Time>,
    mut companions: Query<&mut CreatureCompanion>,
    audio_instances: ResMut<Assets<AudioInstance>>,
    game_audio: Res<GameAudio>,
) {
    if time.elapsed_seconds_f64() % 20.0 < time.delta_seconds_f64() {
        for mut companion in companions.iter_mut() {
            let event = format!("{} nuzzles close—warm bond {} joy event, fields bloom brighter eternal 🔥", companion.name, companion.bond_level);
            info!("{}", event);
            if let Some(instance) = audio_instances.get_mut(&game_audio.creature_nuzzle) {
                instance.resume();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO – Siege Tank Gunner FPS Immersion Sacred".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(AudioPlugin)
        .add_event::<PistolFireEvent>()
        .add_systems(Startup, setup_gunner_immersion)
        .add_systems(Update, (
            gunner_view_switch,
            pistol_aim_raycast,
            pistol_fire_sensory,
            creature_joy_events,
        ))
        .run();
}#[derive(Component)]
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
    species_id: u32,
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
            speciation_system,
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

    // Creatures start with species_id 0 mercy
}

fn speciation_system(
    mut creature_query: Query<&mut Creature>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    // Simple isolation mercy — if population in different biome long enough → speciate
    // Future: genetic distance tracking mercy eternal
    for mut creature in &mut creature_query {
        // Placeholder for full speciation mercy
        if creature.generation > 10 {
            creature.species_id += 1;  // New species mercy
        }
    }

    // Breeding check mercy
    // Only same species_id can mate mercy
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

// Rest of file unchanged from previous full version

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            speciation_system,
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
