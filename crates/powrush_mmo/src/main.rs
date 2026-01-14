use bevy::prelude::*;
use bevy::render::experimental::occlusion_culling::OcclusionCullingPlugin;  // Native GPU occlusion eternal
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin as KiraAudioPlugin};
use bevy_kira_audio::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::RenetServerPlugin;
use renet::{RenetClient, RenetServer, ConnectionConfig};
use mercy_core::PhiloticHive;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;

const LOD_HIGH_THRESHOLD: f32 = 50.0;
const LOD_LOW_THRESHOLD: f32 = 500.0;  // Increased — GPU occlusion handles close hidden mercy

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
    .add_plugins(OcclusionCullingPlugin)  // Native GPU occlusion culling eternal supreme
    .add_plugins(MultiplayerReplicationPlugin);

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
            remote_interpolation,
            entity_culling_optimization,  // Complementary far distance mercy
            lod_mesh_swap,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(42);

    let ground_mesh = meshes.add(shape::Plane::from_size(1000.0).into());
    let ground_material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

    commands.spawn(PbrBundle {
        mesh: ground_mesh,
        material: ground_material,
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        visibility: Visibility::Visible,
        ..default()
    });

    let high_mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 5 }));
    let low_mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
        Player,
        Predicted,
        Velocity(Vec3::ZERO),
        PositionHistory { buffer: VecDeque::new() },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    let resource_material = materials.add(Color::rgb(1.0, 0.8, 0.2).into());
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(-500.0..500.0);
        let z = rng.gen_range(-500.0..500.0);
        let y = perlin.get([x as f64 / 100.0, z as f64 / 100.0]) as f32 * 5.0;

        commands.spawn((
            PbrBundle {
                mesh: high_mesh.clone(),
                material: resource_material.clone(),
                transform: Transform::from_xyz(x, y + 2.0, z),
                visibility: Visibility::Visible,
                ..default()
            },
            Resource,
            LodEntity {
                high_mesh: high_mesh.clone(),
                low_mesh: low_mesh.clone(),
            },
        ));
    }
}

// player_movement, remote_interpolation, emotional_resonance_particles, granular_ambient_evolution unchanged

fn entity_culling_optimization(
    player_query: Query<&Transform, With<Player>>,
    mut cullable_query: Query<(&Transform, &mut Visibility), (With<Resource> | Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        for (transform, mut visibility) in &mut cullable_query {
            let dist_sq = (transform.translation - player_pos).length_squared();
            visibility.visible = dist_sq < (LOD_LOW_THRESHOLD * LOD_LOW_THRESHOLD);
        }
    }
}

fn lod_mesh_swap(
    player_query: Query<&Transform, With<Player>>,
    mut lod_query: Query<(&Transform, &LodEntity, &mut Handle<Mesh>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        for (transform, lod, mut mesh) in &mut lod_query {
            let dist_sq = (transform.translation - player_pos).length_squared();
            let target = if dist_sq < (LOD_HIGH_THRESHOLD * LOD_HIGH_THRESHOLD) {
                &lod.high_mesh
            } else {
                &lod.low_mesh
            };
            if mesh.id() != target.id() {
                *mesh = target.clone();
            }
        }
    }
}

#[derive(Component)]
struct LodEntity {
    high_mesh: Handle<Mesh>,
    low_mesh: Handle<Mesh>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Predicted;

#[derive(Component)]
struct Velocity(pub Vec3);

#[derive(Component)]
struct PositionHistory {
    pub buffer: VecDeque<(Vec3, f64)>,
}

#[derive(Component)]
struct Resource;

#[derive(Component)]
struct EmotionalParticle;

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            remote_interpolation,
            entity_culling_optimization,
            lod_mesh_swap,
        ));
    }
}
