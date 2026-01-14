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
use crate::procedural_music::{ultimate_fm_synthesis, AdsrEnvelope};
use crate::granular_ambient::spawn_pure_procedural_granular_ambient;
use crate::vector_synthesis::vector_wavetable_synthesis;
use crate::networking::MultiplayerReplicationPlugin;

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
    breeding_selection: [Option<Entity>; 2],
}

#[derive(Component)]
struct Creature {
    creature_type: CreatureType,
    state: CreatureState,
    wander_timer: f32,
    age: f32,
    health: f32,
    dna: CreatureDNA,
    tamed: bool,
    owner: Option<Entity>,
}

#[derive(Clone, Copy)]
struct CreatureDNA {
    speed: f32,
    size: f32,
    camouflage: f32,
    aggression: f32,
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
    Dead,
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
    .add_plugins(EguiPlugin)
    .add_plugins(MultiplayerReplicationPlugin)
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
            weather_system,
            creature_behavior_cycle,
            creature_evolution_system,
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
            breeding_selection: [None, None],
        },
        Predicted,
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        PositionHistory { buffer: VecDeque::new() },
    ));
}

fn player_inventory_ui(
    mut contexts: EguiContexts,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    creature_query: Query<&Creature>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        if let Ok(mut player) = player_query.get_single_mut() {
            player.show_inventory = !player.show_inventory;
        }
    }

    if let Ok(player) = player_query.get_single() {
        if player.show_inventory {
            egui::Window::new("Creature Inventory — Mercy Eternal")
                .resizable(true)
                .show(contexts.ctx_mut(), |ui| {
                    ui.heading(format!("Tamed: {}", player.tamed_creatures.len()));

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for &creature_entity in &player.tamed_creatures {
                            if let Ok(creature) = creature_query.get(creature_entity) {
                                let selected1 = player.breeding_selection[0] == Some(creature_entity);
                                let selected2 = player.breeding_selection[1] == Some(creature_entity);
                                let selected = selected1 || selected2;

                                if ui.selectable_label(selected, format!("{:?} — Age {:.0} days", creature.creature_type, creature.age / 10.0)).clicked() {
                                    if let Ok(mut player) = player_query.get_single_mut() {
                                        if selected1 {
                                            player.breeding_selection[0] = None;
                                        } else if selected2 {
                                            player.breeding_selection[1] = None;
                                        } else if player.breeding_selection[0].is_none() {
                                            player.breeding_selection[0] = Some(creature_entity);
                                        } else if player.breeding_selection[1].is_none() {
                                            player.breeding_selection[1] = Some(creature_entity);
                                        }
                                    }
                                }
                            }
                        }
                    });

                    ui.separator();

                    // Breeding Preview Panel
                    if player.breeding_selection[0].is_some() && player.breeding_selection[1].is_some() {
                        ui.heading("Breeding Preview — Potential Offspring");

                        let parent1 = creature_query.get(player.breeding_selection[0].unwrap()).unwrap();
                        let parent2 = creature_query.get(player.breeding_selection[1].unwrap()).unwrap();

                        let avg_speed = (parent1.dna.speed + parent2.dna.speed) / 2.0;
                        let avg_size = (parent1.dna.size + parent2.dna.size) / 2.0;
                        let avg_camouflage = (parent1.dna.camouflage + parent2.dna.camouflage) / 2.0;
                        let avg_aggression = (parent1.dna.aggression + parent2.dna.aggression) / 2.0;

                        ui.label("Average DNA (with mutation variance):");
                        ui.add(egui::ProgressBar::new(avg_speed / 15.0).text(format!("Speed: {:.1} (±1.0)", avg_speed)));
                        ui.add(egui::ProgressBar::new(avg_size / 2.0).text(format!("Size: {:.2} (±0.2)", avg_size)));
                        ui.add(egui::ProgressBar::new(avg_camouflage).text(format!("Camouflage: {:.2} (±0.1)", avg_camouflage)));
                        ui.add(egui::ProgressBar::new(avg_aggression).text(format!("Aggression: {:.2} (±0.1)", avg_aggression)));

                        if ui.button("Breed Selected").clicked() {
                            // Spawn offspring mercy
                            let child_dna = CreatureDNA {
                                speed: avg_speed + rand::thread_rng().gen_range(-1.0..1.0),
                                size: avg_size + rand::thread_rng().gen_range(-0.2..0.2),
                                camouflage: avg_camouflage + rand::thread_rng().gen_range(-0.1..0.1),
                                aggression: avg_aggression + rand::thread_rng().gen_range(-0.1..0.1),
                            };

                            let spawn_pos = Vec3::new(0.0, 5.0, 0.0);  // Near player placeholder

                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: child_dna.size })),
                                    material: materials.add(Color::rgb(child_dna.camouflage, 0.5, 1.0 - child_dna.camouflage).into()),
                                    transform: Transform::from_translation(spawn_pos),
                                    visibility: Visibility::Visible,
                                    ..default()
                                },
                                Creature {
                                    creature_type: parent1.creature_type,
                                    state: CreatureState::Follow,
                                    wander_timer: 5.0,
                                    age: 0.0,
                                    health: 1.0,
                                    dna: child_dna,
                                    tamed: true,
                                    owner: Some(player_query.single().0),
                                },
                                Velocity(Vec3::ZERO),
                            ));

                            if let Ok(mut player) = player_query.get_single_mut() {
                                player.breeding_selection = [None, None];
                            }
                        }
                    }

                    if ui.button("Close").clicked() {
                        if let Ok(mut player) = player_query.get_single_mut() {
                            player.show_inventory = false;
                        }
                    }
                });
        }
    }
}

// Rest of file unchanged from previous full version (player_movement, creature_behavior_cycle, creature_evolution_system, player_breeding_mechanics simplified, chunk_manager, etc.)

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
            creature_evolution_system,
            player_inventory_ui,
            chunk_manager,
        ));
    }
}
