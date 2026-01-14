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
    dna: CreatureDNA,
    tamed: bool,
    owner: Option<Entity>,
    parent1: Option<u64>,
    parent2: Option<u64>,
    generation: u32,
}

#[derive(Clone, Copy)]
struct CreatureDNA {
    speed: f32,         // 5.0-15.0
    size: f32,          // 0.5-2.0
    camouflage: f32,    // 0.0-1.0
    aggression: f32,    // 0.0-1.0
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
            selected_creature: None,
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
                                let selected = player.selected_creature == Some(creature_entity);
                                if ui.selectable_label(selected, format!("{:?} — Gen {} — Age {:.0} days", creature.creature_type, creature.generation, creature.age / 10.0)).clicked() {
                                    if let Ok(mut player) = player_query.get_single_mut() {
                                        player.selected_creature = Some(creature_entity);
                                    }
                                }
                            }
                        }
                    });

                    ui.separator();

                    if let Some(selected_entity) = player.selected_creature {
                        if let Ok(creature) = creature_query.get(selected_entity) {
                            ui.heading("Evolutionary Graph — Mercy Eternal");

                            let painter = ui.painter();
                            let rect = ui.available_rect_before_wrap();
                            let center_x = rect.center().x;
                            let start_y = rect.top() + 40.0;

                            struct Node {
                                pos: Pos2,
                                creature: Creature,
                            }

                            let mut nodes = Vec::new();
                            let mut current = Some(selected_entity);

                            let mut y_offset = start_y;
                            while let Some(entity) = current {
                                if let Ok(c) = creature_query.get(entity) {
                                    nodes.push(Node {
                                        pos: Pos2::new(center_x, y_offset),
                                        creature: c.clone(),
                                    });
                                    y_offset += 100.0;
                                    current = c.parent1.or(c.parent2).and_then(|id| creature_query.iter().find_map(|(e, cr)| if cr.parent1 == Some(id) || cr.parent2 == Some(id) { Some(e) } else { None }));
                                } else {
                                    break;
                                }
                            }

                            nodes.reverse();  // Root at bottom

                            // Draw edges
                            for i in 0..nodes.len() - 1 {
                                let from = nodes[i].pos;
                                let to = nodes[i + 1].pos;
                                painter.line_segment([from, to], Stroke::new(3.0, Color32::WHITE));
                            }

                            // Draw nodes
                            for node in &nodes {
                                let rect = egui::Rect::from_center_size(node.pos, egui::vec2(220.0, 80.0));
                                painter.rect_filled(rect, 12.0, Color32::from_black_alpha(200));
                                painter.text(node.pos, egui::Align2::CENTER_CENTER, format!("Gen {}\n{:?}\nSpeed: {:.1} | Size: {:.2}\nCamo: {:.2} | Agg: {:.2}", 
                                    node.creature.generation, 
                                    node.creature.creature_type,
                                    node.creature.dna.speed,
                                    node.creature.dna.size,
                                    node.creature.dna.camouflage,
                                    node.creature.dna.aggression
                                ), egui::FontId::proportional(16.0), Color32::WHITE);
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

// Rest of file unchanged from previous full version (setup, player_movement, creature_behavior_cycle, creature_evolution_system with parent tracking + generation increment, player_breeding_mechanics setting parent IDs + generation = max(parent generations) + 1, chunk_manager, etc.)

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
            player_breeding_mechanics,
            player_inventory_ui,
            chunk_manager,
        ));
    }
}
