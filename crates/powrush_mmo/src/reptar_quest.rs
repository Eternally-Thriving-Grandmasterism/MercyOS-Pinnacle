//! crates/powrush_mmo/src/reptar_quest.rs
//! "Reptar on Ice" Rugrats-inspired quest with sound effects mercy eternal supreme immaculate
//! Giant Reptar creature in ice biome, feed to tame, ride for joy/speed + audio philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::main::{Player, Creature, CreatureType, CreatureState, FoodResource, ChildWonderMode};

#[derive(Component)]
pub struct Reptar;

#[derive(Component)]
pub struct ReptarTamed;

pub fn spawn_reptar_quest(
    mut commands: Commands,
    wonder_mode: Query<&ChildWonderMode>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if wonder_mode.get_single().is_ok() {
        let player_pos = player_query.single().translation;

        let reptar_pos = player_pos + Vec3::new(50.0, 0.0, 50.0);

        let reptar = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 5.0, subdivisions: 4 })),
                material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
                transform: Transform::from_translation(reptar_pos).with_scale(Vec3::splat(3.0)),
                visibility: Visibility::Visible,
                ..default()
            },
            Creature {
                creature_type: CreatureType::Deer,
                state: CreatureState::Wander,
                wander_timer: 10.0,
                age: 1000.0,
                health: 1.0,
                hunger: 0.5,
                dna: CreatureDNA {
                    speed: 20.0,
                    size: 3.0,
                    camouflage: 0.3,
                    aggression: 0.1,
                    metabolism: 0.5,
                },
                tamed: false,
                owner: None,
                parent1: None,
                parent2: None,
                generation: 1,
                last_drift_day: 0.0,
            },
            Reptar,
        )).id();

        // Reptar roar on spawn mercy eternal
        let roar: Handle<AudioSource> = asset_server.load("sounds/reptar_roar.ogg");
        audio.play(roar)
            .with_volume(0.8)
            .spatial(true)
            .with_position(reptar_pos);
    }
}

pub fn reptar_taming_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut reptar_query: Query<(Entity, &mut Creature, &Transform), With<Reptar>>,
    food_query: Query<(Entity, &Transform), With<FoodResource>>,
    keyboard_input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let player_pos = player_query.single().translation;

    for (reptar_entity, mut reptar, reptar_transform) in &mut reptar_query {
        let dist_to_player = (reptar_transform.translation - player_pos).length();

        if dist_to_player < 10.0 && keyboard_input.just_pressed(KeyCode::E) {
            for (food_entity, food_transform) in &food_query {
                if (food_transform.translation - reptar_transform.translation).length() < 5.0 {
                    commands.entity(food_entity).despawn();
                    reptar.hunger -= 0.5;
                    if reptar.hunger <= 0.0 {
                        reptar.tamed = true;
                        reptar.state = CreatureState::Follow;
                        commands.entity(reptar_entity).insert(ReptarTamed);

                        // Happy chuff on tame mercy eternal
                        let chuff: Handle<AudioSource> = asset_server.load("sounds/reptar_chuff.ogg");
                        audio.play(chuff)
                            .with_volume(0.7)
                            .spatial(true)
                            .with_position(reptar_transform.translation);
                    }
                    break;
                }
            }
        }
    }
}

pub fn reptar_ride_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    reptar_query: Query<(Entity, &Transform), (With<ReptarTamed>, With<Creature>)>,
    keyboard_input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let (player_entity, player_transform) = player_query.single();

    for (reptar_entity, reptar_transform) in &reptar_query {
        let dist = (player_transform.translation - reptar_transform.translation).length();

        if dist < 8.0 && keyboard_input.just_pressed(KeyCode::R) {
            commands.entity(player_entity).set_parent(reptar_entity);

            // Playful stomp rumble on ride mercy eternal
            let stomp: Handle<AudioSource> = asset_server.load("sounds/reptar_stomp.ogg");
            audio.play(stomp)
                .with_volume(0.9)
                .spatial(true)
                .with_position(reptar_transform.translation);
        }
    }
}

pub struct ReptarQuestPlugin;

impl Plugin for ReptarQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_reptar_quest,
            reptar_taming_system,
            reptar_ride_system,
        ));
    }
}                    }
                    break;
                }
            }
        }
    }
}

pub fn reptar_ride_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    reptar_query: Query<(Entity, &Transform), (With<ReptarTamed>, With<Creature>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (player_entity, player_transform) = player_query.single();

    for (reptar_entity, reptar_transform) in &reptar_query {
        let dist = (player_transform.translation - reptar_transform.translation).length();

        if dist < 8.0 && keyboard_input.just_pressed(KeyCode::R) {
            // Ride Reptar mercy
            commands.entity(player_entity).set_parent(reptar_entity);
            // Speed bonus + joy particles mercy
        }
    }
}

pub struct ReptarQuestPlugin;

impl Plugin for ReptarQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_reptar_quest,
            reptar_taming_system,
            reptar_ride_system,
        ));
    }
}
