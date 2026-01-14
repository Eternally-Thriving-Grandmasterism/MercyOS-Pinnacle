//! crates/powrush_mmo/src/child_wonder_mode.rs
//! Child Wonder Mode — Rugrats-inspired baby perspective mercy eternal supreme immaculate
//! Low camera, wide FOV, saturated colors, playful audio, family harmony bonuses philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct ChildWonderMode;

#[derive(Resource)]
pub struct WonderModeState {
    pub active: bool,
}

pub fn toggle_child_wonder_mode(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<WonderModeState>,
    mut camera_query: Query<&mut Camera>,
    mut audio: Res<Audio>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {  // C for Child mercy
        state.active = !state.active;

        if state.active {
            // Low baby-eye view mercy
            if let Ok(mut camera) = camera_query.get_single_mut() {
                camera.transform.translation.y = 0.8;  // Baby height mercy
                // Wider FOV for child wonder
                // camera.fov = 100.0;  // Future camera component mercy
            }

            // Saturated colors + playful audio pitch mercy
            // Post-process stub — future shader
            audio.set_global_volume(1.2);
            // Play playful background mercy
        } else {
            // Reset to normal mercy
        }
    }
}

pub fn family_harmony_bonus_system(
    state: Res<WonderModeState>,
    player_query: Query<&Player>,
    creature_query: Query<&Creature>,
) {
    if state.active {
        if let Ok(player) = player_query.get_single() {
            let family_count = player.tamed_creatures.len();
            // Harmony bonuses mercy — higher joy level, creature loyalty eternal
        }
    }
}

pub struct ChildWonderPlugin;

impl Plugin for ChildWonderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WonderModeState { active: false })
            .add_systems(Update, (toggle_child_wonder_mode, family_harmony_bonus_system));
    }
}
