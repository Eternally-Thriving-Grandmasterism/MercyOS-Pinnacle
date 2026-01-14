//! crates/powrush_mmo/src/child_wonder_mode.rs
//! Child Wonder Mode — Rugrats-inspired baby perspective + imaginative overlays mercy eternal supreme immaculate
//! Low camera, giant world, colorful sparkles on movement, playful audio pitch philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct ChildWonderMode;

#[derive(Resource)]
pub struct WonderModeState {
    pub active: bool,
}

#[derive(Component)]
pub struct ImaginationSparkle;

pub fn toggle_child_wonder_mode(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<WonderModeState>,
    mut camera_query: Query<&mut Transform, With<FirstPersonCamera>>,
    audio: Res<Audio>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        state.active = !state.active;

        if state.active {
            // Baby-eye view mercy
            if let Ok(mut camera) = camera_query.get_single_mut() {
                camera.translation.y = 0.8;
            }

            // Playful high-pitch background mercy
            audio.set_global_pitch(1.3);
        } else {
            if let Ok(mut camera) = camera_query.get_single_mut() {
                camera.translation.y = 1.6;
            }

            audio.set_global_pitch(1.0);
        }
    }
}

pub fn imaginative_overlays_system(
    state: Res<WonderModeState>,
    player_query: Query<&Velocity, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if state.active {
        let velocity = player_query.single();

        if velocity.0.length() > 0.1 {
            // Colorful sparkle trail mercy eternal
            let mut rng = thread_rng();
            for _ in 0..3 {
                let offset = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(0.0..0.5), rng.gen_range(-0.5..0.5));

                let color = Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.7);

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, ..default() })),
                        material: materials.add(StandardMaterial {
                            base_color: color,
                            emissive: color * 5.0,
                            ..default()
                        }),
                        transform: Transform::from_translation(player_transform.translation + offset),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    ImaginationSparkle,
                    Lifetime(1.0),
                ));
            }
        }

        // Giant ordinary objects mercy — tag furniture/trees with ImaginationScale component
        // In chunk_manager or setup, add ImaginationScale(3.0) to trees/rocks mercy
        // System scales when mode active mercy
    }
}

#[derive(Component)]
pub struct ImaginationScale(pub f32);

pub fn imagination_scale_system(
    state: Res<WonderModeState>,
    mut query: Query<&mut Transform, With<ImaginationScale>>,
) {
    for mut transform in &mut query {
        let scale = if state.active { transform.scale = Vec3::splat(query.get_component::<ImaginationScale>().unwrap().0) } else { Vec3::ONE };
        transform.scale = scale;
    }
}

#[derive(Component)]
struct Lifetime(f32);

pub fn lifetime_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub struct ChildWonderPlugin;

impl Plugin for ChildWonderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WonderModeState { active: false })
            .add_systems(Update, (
                toggle_child_wonder_mode,
                imaginative_overlays_system,
                imagination_scale_system,
                lifetime_system,
                family_harmony_bonus_system,
            ));
    }
}
