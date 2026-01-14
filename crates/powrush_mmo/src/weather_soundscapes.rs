//! crates/powrush_mmo/src/weather_soundscapes.rs
//! Weather soundscapes integration mercy eternal supreme immaculate
//! Dynamic ambient loops + effects by current weather + biome philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::main::{WeatherManager, Weather, Chunk, Player};

#[derive(Component)]
pub struct WeatherAmbience;

pub fn weather_soundscapes_system(
    mut commands: Commands,
    weather: Res<WeatherManager>,
    chunk_query: Query<(&Chunk, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut ambience_query: Query<Entity, With<WeatherAmbience>>,
) {
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

    // Despawn old ambience mercy
    for entity in &ambience_query {
        commands.entity(entity).despawn();
    }

    // Spawn new ambience mercy eternal
    let ambience_entity = commands.spawn(WeatherAmbience).id();

    match (weather.current, current_biome) {
        (Weather::Rain, Biome::Forest) => {
            let rain: Handle<AudioSource> = asset_server.load("sounds/rain_forest.ogg");
            audio.play(rain)
                .with_volume(weather.intensity)
                .looped()
                .spatial(true)
                .with_position(player_pos);
        }
        (Weather::Rain, Biome::Plains) => {
            let rain: Handle<AudioSource> = asset_server.load("sounds/rain_plains.ogg");
            audio.play(rain)
                .with_volume(weather.intensity)
                .looped()
                .spatial(true)
                .with_position(player_pos);
        }
        (Weather::Snow, Biome::Tundra) => {
            let snow: Handle<AudioSource> = asset_server.load("sounds/snow_crunch.ogg");
            audio.play(snow)
                .with_volume(weather.intensity * 0.6)
                .looped()
                .spatial(true)
                .with_position(player_pos);
        }
        (Weather::Storm, _) => {
            let wind: Handle<AudioSource> = asset_server.load("sounds/storm_wind.ogg");
            audio.play(wind)
                .with_volume(weather.intensity)
                .looped()
                .spatial(true)
                .with_position(player_pos);

            // Thunder one-shots mercy
            if rand::thread_rng().gen_bool((0.05 * weather.intensity) as f64) {
                let thunder: Handle<AudioSource> = asset_server.load("sounds/thunder.ogg");
                audio.play(thunder).with_volume(0.8);
            }
        }
        _ => {
            // Default calm ambience mercy
            let calm: Handle<AudioSource> = asset_server.load("sounds/calm_ambience.ogg");
            audio.play(calm)
                .with_volume(0.3)
                .looped()
                .spatial(true)
                .with_position(player_pos);
        }
    }
}

pub struct WeatherSoundscapesPlugin;

impl Plugin for WeatherSoundscapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, weather_soundscapes_system);
    }
}
