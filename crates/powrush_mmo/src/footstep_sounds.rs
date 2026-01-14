//! crates/powrush_mmo/src/footstep_sounds.rs
//! Footstep sounds integration mercy eternal supreme immaculate
//! Dynamic surface + biome + weather footsteps with spatial variation philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::main::{Player, WeatherManager, Chunk};

#[derive(Component)]
pub struct FootstepTimer(Timer);

pub fn footstep_setup_system(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).insert(FootstepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    }
}

pub fn footstep_sounds_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&Transform, &Velocity, &mut FootstepTimer), With<Player>>,
    chunk_query: Query<(&Chunk, &Transform)>,
    weather: Res<WeatherManager>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (transform, velocity, mut timer) in &mut player_query {
        timer.0.tick(time.delta());

        let speed = velocity.0.length();
        if speed > 0.5 && timer.0.just_finished() {
            let foot_pos = transform.translation + Vec3::new(0.0, -0.8, 0.0);

            // Raycast down for ground mercy
            let ray = Ray::new(foot_pos.into(), Vec3::NEG_Y.into());
            if let Some((_, toi)) = rapier_context.cast_ray(ray.origin, ray.dir, 1.0, true, QueryFilter::default()) {
                let hit_point = ray.origin + ray.dir * toi;

                // Find chunk + voxel mercy
                let mut surface_type = 3;  // Default grass mercy
                for (chunk, chunk_transform) in &chunk_query {
                    let local = hit_point - chunk_transform.translation.into();
                    if local.x.abs() < CHUNK_SIZE as f32 / 2.0 && local.z.abs() < CHUNK_SIZE as f32 / 2.0 {
                        let lx = (local.x as u32).min(CHUNK_SIZE - 1);
                        let ly = (local.y as u32).min(CHUNK_SIZE - 1);
                        let lz = (local.z as u32).min(CHUNK_SIZE - 1);

                        let index = ChunkShape::linearize([lx, ly, lz]) as usize;
                        surface_type = chunk.voxels[index];
                        break;
                    }
                }

                // Biome + weather modifiers mercy
                let mut volume = 0.6;
                let mut pitch = 1.0;
                let sound_path = match (surface_type, weather.current) {
                    (1, _) => {  // Rock mercy
                        volume = 0.8;
                        "sounds/footstep_rock.ogg"
                    }
                    (2, Weather::Rain) => {  // Mud wet mercy
                        volume = 0.9;
                        pitch = 0.8;
                        "sounds/footstep_mud.ogg"
                    }
                    (2, _) => "sounds/footstep_dirt.ogg",
                    (3, _) => "sounds/footstep_grass.ogg",
                    (4, Weather::Snow) => {  // Snow mercy
                        volume = 0.4;
                        pitch = 1.2;
                        "sounds/footstep_snow.ogg"
                    }
                    (4, _) => "sounds/footstep_ice.ogg",
                    _ => "sounds/footstep_grass.ogg",
                };

                let footstep: Handle<AudioSource> = asset_server.load(sound_path);
                audio.play(footstep)
                    .with_volume(volume * weather.intensity)
                    .with_pitch(pitch + rand::thread_rng().gen_range(-0.1..0.1))
                    .spatial(true)
                    .with_position(foot_pos);
            }
        }
    }
}

pub struct FootstepSoundsPlugin;

impl Plugin for FootstepSoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, footstep_setup_system)
            .add_systems(Update, footstep_sounds_system);
    }
}
