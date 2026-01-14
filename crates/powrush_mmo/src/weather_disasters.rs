//! crates/powrush_mmo/src/weather_disasters.rs
//! Weather-driven disasters integration mercy eternal supreme immaculate
//! High-intensity weather + biome synergy spawns cataclysmic events philotic mercy

use bevy::prelude::*;
use crate::main::{WeatherManager, Weather, Biome, Player, Creature};

#[derive(Component)]
pub struct ActiveDisaster {
    pub disaster_type: DisasterType,
    pub duration: f32,
    pub intensity: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DisasterType {
    Flood,
    Blizzard,
    Sandstorm,
    Wildfire,
}

pub fn weather_disaster_trigger_system(
    mut commands: Commands,
    weather: Res<WeatherManager>,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    if weather.intensity < 0.8 {
        return;  // Only high intensity mercy
    }

    let player_pos = player_query.single().translation;

    // Find current biome mercy
    let mut current_biome = Biome::Plains;
    for (chunk, chunk_transform) in &chunk_query {
        let local = player_pos - chunk_transform.translation;
        if local.x.abs() < CHUNK_SIZE as f32 / 2.0 && local.z.abs() < CHUNK_SIZE as f32 / 2.0 {
            current_biome = chunk.biome;
            break;
        }
    }

    let disaster = match (weather.current, current_biome) {
        (Weather::Rain, Biome::Ocean) => Some(DisasterType::Flood),
        (Weather::Snow, Biome::Tundra) => Some(DisasterType::Blizzard),
        (Weather::Storm, Biome::Desert) => Some(DisasterType::Sandstorm),
        (Weather::Clear, Biome::Forest) if weather.intensity > 0.9 => Some(DisasterType::Wildfire),  // Heat wave mercy
        _ => None,
    };

    if let Some(disaster_type) = disaster {
        if rand::thread_rng().gen_bool(0.05) {  // Rare mercy
            commands.spawn(ActiveDisaster {
                disaster_type,
                duration: 300.0,
                intensity: weather.intensity,
            });
        }
    }
}

pub fn weather_disaster_effects_system(
    mut commands: Commands,
    disaster_query: Query<(Entity, &ActiveDisaster)>,
    mut creature_query: Query<&mut Creature>,
    player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    for (entity, disaster) in &disaster_query {
        let mut duration = disaster.duration;
        duration -= time.delta_seconds();

        // Effects mercy eternal
        match disaster.disaster_type {
            DisasterType::Flood => {
                // Water damage + slow mercy
                for mut creature in &mut creature_query {
                    creature.health -= time.delta_seconds() * 0.05 * disaster.intensity;
                }
            }
            DisasterType::Blizzard => {
                // Cold damage + vision fog mercy
                // Spawn snow particles heavy
            }
            DisasterType::Sandstorm => {
                // Vision zero + damage mercy
            }
            DisasterType::Wildfire => {
                // Fire damage spread mercy
            }
        }

        if duration <= 0.0 {
            commands.entity(entity).despawn();
            // Recovery quest spawn mercy
        }
    }
}

pub struct WeatherDisastersPlugin;

impl Plugin for WeatherDisastersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            weather_disaster_trigger_system,
            weather_disaster_effects_system,
        ));
    }
}
