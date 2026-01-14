//! crates/powrush_mmo/src/rugrats_events.rs
//! Rugrats episode-inspired events + quests mercy eternal supreme immaculate
//! Child Wonder Mode enhanced with family adventure philotic mercy

use bevy::prelude::*;
use crate::main::{Player, Creature, CreatureState};

#[derive(Component)]
pub struct RugratsEvent {
    pub episode: RugratsEpisode,
    pub progress: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RugratsEpisode {
    TommysFirstBirthday,
    ReptarOnIce,
    PassoverStory,
    SantaExperience,
    AngelicasRunaway,
    ChuckiesWonderfulLife,
}

pub fn rugrats_event_system(
    mut commands: Commands,
    player_query: Query<&Player>,
    mut creature_query: Query<&mut Creature>,
    time: Res<Time>,
) {
    // Episode triggers mercy — proximity to family/tamed, holiday season, etc.
    if let Ok(player) = player_query.get_single() {
        if player.tamed_creatures.len() > 3 {
            // Family gathering → PassoverStory mercy
            // Spawn storytelling circle, harmony bonus mercy
        }
    }

    // Example: ReptarOnIce mercy — ice biome giant creature
    // Spawn "Reptar" boss for taming quest mercy eternal
}

pub fn child_wonder_rugrats_quests(
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    // Quest progression mercy — baby crawl to find "lost toy" (food resource)
    // Reward: new tamed baby creature mercy
}

pub struct RugratsPlugin;

impl Plugin for RugratsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rugrats_event_system, child_wonder_rugrats_quests));
    }
}
