//! crates/powrush_mmo/src/didi_quest.rs
//! "Didi Pickup" Rugrats-inspired quest mercy eternal supreme immaculate
//! Giant Didi NPC spawns in Child Wonder Mode, approach to hug/reunite for family joy philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::main::{Player, ChildWonderMode};

#[derive(Component)]
pub struct DidiNpc;

#[derive(Component)]
pub struct DidiQuestActive;

pub fn spawn_didi_quest(
    mut commands: Commands,
    wonder_mode: Query<&ChildWonderMode>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if wonder_mode.get_single().is_ok() && commands.query::<&DidiQuestActive>().iter().next().is_none() {
        let player_pos = player_query.single().translation;

        let didi_pos = player_pos + Vec3::new(80.0, 0.0, 80.0);  // Distant adventure mercy

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(1.0, 0.7, 0.8).into()),
                transform: Transform::from_translation(didi_pos).with_scale(Vec3::splat(8.0)),  // Giant mom mercy
                visibility: Visibility::Visible,
                ..default()
            },
            DidiNpc,
            DidiQuestActive,
        ));

        // Didi calling voice mercy eternal
        let didi_call: Handle<AudioSource> = asset_server.load("sounds/didi_calling.ogg");
        audio.play(didi_call)
            .with_volume(0.8)
            .spatial(true)
            .with_position(didi_pos)
            .looped();
    }
}

pub fn didi_pickup_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    didi_query: Query<(Entity, &Transform), With<DidiNpc>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let player_pos = player_query.single().translation;

    for (didi_entity, didi_transform) in &didi_query {
        let dist = (player_pos - didi_transform.translation).length();

        if dist < 10.0 {
            // Hug reunion mercy eternal
            let hug: Handle<AudioSource> = asset_server.load("sounds/didi_hug.ogg");
            audio.play(hug).with_volume(1.0);

            // Joy particles + family harmony bonus mercy
            // Spawn massive joy sparkles, temporary speed boost

            commands.entity(didi_entity).despawn();
        }
    }
}

pub struct DidiQuestPlugin;

impl Plugin for DidiQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_didi_quest, didi_pickup_system));
    }
}
