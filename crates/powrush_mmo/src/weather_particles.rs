//! crates/powrush_mmo/src/weather_particles.rs
//! Weather particles integration mercy eternal supreme immaculate
//! Dynamic rain drops, snow flakes, fog volumes, storm lightning + wind gusts philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};
use crate::main::{WeatherManager, Weather, Player, ChildWonderMode};

#[derive(Component)]
pub struct WeatherParticle;

#[derive(Component)]
pub struct Lifetime(f32);

pub fn weather_particles_system(
    mut commands: Commands,
    weather: Res<WeatherManager>,
    player_query: Query<&Transform, With<Player>>,
    wonder_mode: Query<&ChildWonderMode>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        let intensity = weather.intensity;
        let is_wonder = wonder_mode.get_single().is_ok();

        match weather.current {
            Weather::Rain => {
                let drop_count = (20.0 * intensity) as usize;
                let drop_mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 0.05, subdivisions: 2 }));
                let drop_material = materials.add(StandardMaterial {
                    base_color: Color::rgba(0.5, 0.7, 1.0, 0.6),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                });

                for _ in 0..drop_count {
                    let x = player_pos.x + thread_rng().gen_range(-30.0..30.0);
                    let z = player_pos.z + thread_rng().gen_range(-30.0..30.0);
                    let y = player_pos.y + 20.0 + thread_rng().gen_range(0.0..10.0);

                    commands.spawn((
                        PbrBundle {
                            mesh: drop_mesh.clone(),
                            material: drop_material.clone(),
                            transform: Transform::from_xyz(x, y, z),
                            ..default()
                        },
                        WeatherParticle,
                        Lifetime(5.0),
                    ));
                }
            }
            Weather::Snow => {
                let flake_count = (15.0 * intensity) as usize;
                let flake_mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 1 }));
                let flake_material = materials.add(Color::rgb(0.95, 0.95, 1.0));

                for _ in 0..flake_count {
                    let x = player_pos.x + thread_rng().gen_range(-40.0..40.0);
                    let z = player_pos.z + thread_rng().gen_range(-40.0..40.0);
                    let y = player_pos.y + 25.0;

                    commands.spawn((
                        PbrBundle {
                            mesh: flake_mesh.clone(),
                            material: flake_material.clone(),
                            transform: Transform::from_xyz(x, y, z),
                            ..default()
                        },
                        WeatherParticle,
                        Lifetime(10.0),
                    ));
                }
            }
            Weather::Fog => {
                // Fog volume mercy — placeholder low alpha cubes
                let fog_material = materials.add(StandardMaterial {
                    base_color: Color::rgba(0.8, 0.8, 0.8, 0.1 * intensity),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                });

                for _ in 0..(10.0 * intensity) as usize {
                    let x = player_pos.x + thread_rng().gen_range(-20.0..20.0);
                    let y = player_pos.y + thread_rng().gen_range(0.0..5.0);
                    let z = player_pos.z + thread_rng().gen_range(-20.0..20.0);

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
                            material: fog_material.clone(),
                            transform: Transform::from_xyz(x, y, z),
                            ..default()
                        },
                        WeatherParticle,
                        Lifetime(8.0),
                    ));
                }
            }
            Weather::Storm => {
                // Lightning flash mercy
                if thread_rng().gen_bool((0.05 * intensity) as f64) {
                    // Flash screen white mercy — future post-process
                }

                // Wind gust particles mercy
            }
            Weather::Clear => {
                // Sunny sparkles in wonder mode mercy
                if is_wonder {
                    for _ in 0..5 {
                        let offset = Vec3::new(
                            thread_rng().gen_range(-10.0..10.0),
                            thread_rng().gen_range(5.0..15.0),
                            thread_rng().gen_range(-10.0..10.0),
                        );

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.2, ..default() })),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(1.0, 1.0, 0.8),
                                    emissive: Color::rgb(1.0, 1.0, 0.8) * 10.0,
                                    ..default()
                                }),
                                transform: Transform::from_translation(player_pos + offset),
                                ..default()
                            },
                            WeatherParticle,
                            Lifetime(2.0),
                        ));
                    }
                }
            }
        }
    }
}

pub fn weather_particle_movement_system(
    mut query: Query<&mut Transform, With<WeatherParticle>>,
    weather: Res<WeatherManager>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        match weather.current {
            Weather::Rain => {
                transform.translation.y -= 20.0 * time.delta_seconds();
            }
            Weather::Snow => {
                transform.translation.y -= 3.0 * time.delta_seconds();
                transform.translation.x += (time.elapsed_seconds().sin() * 0.5) * time.delta_seconds();
            }
            Weather::Fog => {
                // Drift slowly mercy
            }
            _ => {}
        }
    }
}

pub fn lifetime_cleanup_system(
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

pub struct WeatherParticlesPlugin;

impl Plugin for WeatherParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            weather_particles_system,
            weather_particle_movement_system,
            lifetime_cleanup_system,
        ));
    }
}
