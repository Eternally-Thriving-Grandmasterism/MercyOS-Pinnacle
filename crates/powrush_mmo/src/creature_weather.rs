//! crates/powrush_mmo/src/creature_weather.rs
//! Creature weather reactions mercy eternal supreme immaculate
//! Dynamic state + hunger/health + sound adaptation by current weather philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::main::{Creature, CreatureState, WeatherManager, Weather};

pub fn creature_weather_reactions_system(
    mut creature_query: Query<&mut Creature>,
    weather: Res<WeatherManager>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for mut creature in &mut creature_query {
        match weather.current {
            Weather::Rain => {
                // Seek shelter mercy
                if rand::thread_rng().gen_bool(0.1) {
                    creature.state = CreatureState::Flee;
                }
                creature.hunger += time.delta_seconds() * 0.05;  // Wet slower foraging mercy

                // Rain shiver sound mercy
                let shiver: Handle<AudioSource> = asset_server.load("sounds/creature_shiver.ogg");
                audio.play(shiver).with_volume(0.4);
            }
            Weather::Snow => {
                // Huddle warm mercy
                creature.state = CreatureState::Sleep;
                creature.health -= time.delta_seconds() * 0.02;  // Cold mercy

                // Snow crunch mercy
                let crunch: Handle<AudioSource> = asset_server.load("sounds/snow_crunch.ogg");
                audio.play(crunch).with_volume(0.3);
            }
            Weather::Storm => {
                // Flee storm mercy
                creature.state = CreatureState::Flee;

                // Storm fear sound mercy
                let fear: Handle<AudioSource> = asset_server.load("sounds/creature_fear.ogg");
                audio.play(fear).with_volume(0.6);
            }
            Weather::Fog => {
                // Camouflage bonus mercy
                creature.dna.camouflage += 0.2;

                // Fog mysterious sound mercy
                let mist: Handle<AudioSource> = asset_server.load("sounds/fog_mist.ogg");
                audio.play(mist).with_volume(0.2);
            }
            Weather::Clear => {
                // Normal behavior mercy
                // Sunny happy sound mercy
                let happy: Handle<AudioSource> = asset_server.load("sounds/creature_happy.ogg");
                audio.play(happy).with_volume(0.4);
            }
        }
    }
}

pub struct CreatureWeatherPlugin;

impl Plugin for CreatureWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, creature_weather_reactions_system);
    }
}
