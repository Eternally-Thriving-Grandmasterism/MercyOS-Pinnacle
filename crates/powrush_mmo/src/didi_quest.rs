//! crates/powrush_mmo/src/didi_quest.rs
//! "Didi Pickup" Rugrats-inspired quest with full visual model mercy eternal supreme immaculate
//! Giant loving mom multi-part avatar, approach for hug reunion joy philotic mercy

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

        let didi_pos = player_pos + Vec3::new(80.0, 0.0, 80.0);

        let giant_scale = if wonder_mode.get_single().is_ok() { 8.0 } else { 1.8 };

        let didi_root = commands.spawn((
            Transform::from_translation(didi_pos),
            GlobalTransform::default(),
            Visibility::Visible,
            DidiNpc,
            DidiQuestActive,
        )).id();

        // Purple dress body mercy
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.6, 0.2, 0.8).into()),  // Purple dress mercy
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.5 * giant_scale)),
                ..default()
            },
            PlayerBodyPart,
        )).set_parent(didi_root);

        // Skin head mercy
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
                material: materials.add(Color::rgb(0.9, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 1.8 * giant_scale, 0.0).with_scale(Vec3::splat(0.8 * giant_scale)),
                ..default()
            },
            PlayerBodyPart,
        )).set_parent(didi_root);

        // Orange curly hair mercy — simple cone stack
        let hair_material = materials.add(Color::rgb(1.0, 0.5, 0.0).into());
        for i in 0..5 {
            let offset = i as f32 * 0.2 * giant_scale;
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cone { radius: 0.6 * giant_scale, height: 0.4 * giant_scale, resolution: 16 })),
                    material: hair_material.clone(),
                    transform: Transform::from_xyz(0.0, 1.8 * giant_scale + offset, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                PlayerBodyPart,
            )).set_parent(didi_root);
        }

        // Arms mercy
        let arm_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 0.2 * giant_scale, height: 1.2 * giant_scale, resolution: 16 }));
        commands.spawn((
            PbrBundle {
                mesh: arm_mesh.clone(),
                material: skin_material.clone(),
                transform: Transform::from_xyz(-0.8 * giant_scale, 0.5 * giant_scale, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            PlayerBodyPart,
        )).set_parent(didi_root);

        commands.spawn((
            PbrBundle {
                mesh: arm_mesh,
                material: skin_material.clone(),
                transform: Transform::from_xyz(0.8 * giant_scale, 0.5 * giant_scale, 0.0).with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            PlayerBodyPart,
        )).set_parent(didi_root);

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

            // Massive joy particles mercy
            // Spawn sparkle storm + harmony bonus

            commands.entity(didi_entity).despawn_recursive();
        }
    }
}

pub struct DidiQuestPlugin;

impl Plugin for DidiQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_didi_quest, didi_pickup_system));
    }
}
