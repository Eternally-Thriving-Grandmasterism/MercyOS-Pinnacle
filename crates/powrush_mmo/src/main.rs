// In multi_chain_ik_system mercy — add finger chains with natural constraints
fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut finger_query: Query<&mut Transform, Or<(With<LeftIndexProximal>, With<LeftIndexIntermediate>, With<LeftIndexDistal>, With<RightIndexProximal>, With<RightIndexIntermediate>, With<RightIndexDistal>)>>,
    finger_tip_query: Query<&Transform, Or<(With<LeftIndexTip>, With<RightIndexTip>)>>,
) {
    let player_transform = player_query.single();

    // Example left index finger IK mercy
    if let (Ok(mut proximal), Ok(mut intermediate), Ok(mut distal), Ok(tip)) = (
        finger_query.get_component_mut::<Transform>(/* left_index_proximal */),
        finger_query.get_component_mut::<Transform>(/* left_index_intermediate */),
        finger_query.get_component_mut::<Transform>(/* left_index_distal */),
        finger_tip_query.get_single().ok(),
    ) {
        let wrist = player_transform.translation + Vec3::new(-0.4, -0.8, 0.0);  // Approximate wrist
        let mut positions = [
            wrist,
            proximal.translation,
            intermediate.translation,
            distal.translation,
            tip.translation,
        ];

        let lengths = [0.15, 0.1, 0.08];  // Phalanx lengths mercy

        // Natural finger constraints mercy eternal
        let constraints = [
            (-0.1, std::f32::consts::FRAC_PI_2),     // Proximal curl only mercy
            (0.0, std::f32::consts::FRAC_PI_2 * 1.1), // Intermediate more flexible
            (0.0, std::f32::consts::FRAC_PI_2),       // Distal curl mercy
        ];

        fabrik_constrained(&mut positions, &lengths, &constraints, tip.translation, 0.01, 10);

        proximal.translation = positions[1];
        intermediate.translation = positions[2];
        distal.translation = positions[3];

        proximal.look_at(positions[2], Vec3::Y);
        intermediate.look_at(positions[3], Vec3::Y);
        distal.look_at(tip.translation, Vec3::Y);
    }

    // Right index + other fingers symmetric mercy

    // Arms, spine, legs unchanged mercy
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
struct SpineRoot;

#[derive(Component)]
struct SpineLower;

#[derive(Component)]
struct SpineMid;

#[derive(Component)]
struct SpineUpper;

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

    // Full multi-chain body avatar mercy — arms with wrist rotation limits
    let arm_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 0.1, height: 0.8, resolution: 16 }));
    let forearm_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 0.09, height: 0.8, resolution: 16 }));
    let hand_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.2 }));

    let skin_material = materials.add(Color::rgb(0.9, 0.7, 0.6).into());

    // Left arm chain mercy
    let left_upper_arm = commands.spawn((
        PbrBundle {
            mesh: arm_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(-0.3, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftUpperArm,
        PlayerBodyPart,
    )).id();

    let left_forearm = commands.spawn((
        PbrBundle {
            mesh: forearm_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftForearm,
        PlayerBodyPart,
    )).id();

    let left_hand = commands.spawn((
        PbrBundle {
            mesh: hand_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        LeftHandTarget,
    )).id();

    commands.entity(left_upper_arm).push_children(&[left_forearm]);
    commands.entity(left_forearm).push_children(&[left_hand]);
    commands.entity(player_body).push_children(&[left_upper_arm]);

    // Right arm symmetric mercy
    let right_upper_arm = commands.spawn((
        PbrBundle {
            mesh: arm_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.3, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
            visibility: Visibility::Visible,
            ..default()
        },
        RightUpperArm,
        PlayerBodyPart,
    )).id();

    let right_forearm = commands.spawn((
        PbrBundle {
            mesh: forearm_mesh.clone(),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightForearm,
        PlayerBodyPart,
    )).id();

    let right_hand = commands.spawn((
        PbrBundle {
            mesh: hand_mesh,
            material: skin_material,
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        RightHandTarget,
    )).id();

    commands.entity(right_upper_arm).push_children(&[right_forearm]);
    commands.entity(right_forearm).push_children(&[right_hand]);
    commands.entity(player_body).push_children(&[right_upper_arm]);

    // Spine and legs chains mercy (as before)

    // Head mercy
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

fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut arm_query: Query<&mut Transform, Or<(With<LeftUpperArm>, With<LeftForearm>, With<RightUpperArm>, With<RightForearm>)>>,
    hand_target_query: Query<&Transform, Or<(With<LeftHandTarget>, With<RightHandTarget>)>>,
) {
    let player_transform = player_query.single();

    // Left arm TRIK + wrist rotation limit mercy
    if let (Ok(mut left_upper), Ok(mut left_forearm), Ok(left_hand)) = (
        arm_query.get_component_mut::<Transform>(/* left_upper_arm */),
        arm_query.get_component_mut::<Transform>(/* left_forearm */),
        hand_target_query.get_single().ok(),
    ) {
        let shoulder = player_transform.translation + Vec3::new(-0.3, 0.0, 0.0);
        let target = left_hand.translation;

        let (elbow, _) = trik_two_bone(shoulder, 0.4, 0.4, target);

        left_upper.translation = (shoulder + elbow) / 2.0;
        left_upper.look_at(elbow, Vec3::Y);

        left_forearm.translation = (elbow + target) / 2.0;
        left_forearm.look_at(target, Vec3::Y);

        // Wrist rotation limit mercy — pronation/supination ±80° + flexion/extension
        let forearm_dir = (target - elbow).normalize_or_zero();
        let current_hand_forward = left_hand.forward();

        // Project to forearm plane mercy
        let projected = current_hand_forward - forearm_dir * current_hand_forward.dot(forearm_dir);
        let clamped = projected.normalize_or_zero();

        // Limit twist angle mercy
        let twist_angle = current_hand_forward.angle_between(clamped);
        if twist_angle > std::f32::consts::FRAC_PI_2 * 0.9 {  // ±80° mercy
            let correction = Quat::from_rotation_arc(current_hand_forward, clamped);
            left_hand.rotation = correction * left_hand.rotation;
        }
    }

    // Right arm symmetric mercy

    // Spine and legs unchanged mercy
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
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
