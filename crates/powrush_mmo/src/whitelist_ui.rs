//! crates/powrush_mmo/src/whitelist_ui.rs
//! Dynamic whitelist management UI mercy eternal supreme immaculate
//! EGUI runtime phrase add/remove for MercyShield philotic mercy

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::mercy_shield::MercyShieldConfig;

#[derive(Resource, Default)]
pub struct WhitelistUIState {
    pub show_window: bool,
    pub new_phrase: String,
}

pub fn whitelist_ui_system(
    mut contexts: EguiContexts,
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: Local<WhitelistUIState>,
    mut config: ResMut<MercyShieldConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::W) {
        ui_state.show_window = !ui_state.show_window;
    }

    if ui_state.show_window {
        egui::Window::new("MercyShield Whitelist — Trusted Phrases Mercy Eternal")
            .resizable(true)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Trusted Phrases (bypass scam filter mercy)");

                // List current phrases mercy
                let mut to_remove = None;
                for (i, phrase) in config.whitelist_phrases.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(phrase);
                        if ui.button("Remove").clicked() {
                            to_remove = Some(phrase.clone());
                        }
                    });
                }

                if let Some(phrase) = to_remove {
                    config.whitelist_phrases.remove(&phrase);
                }

                // Add new phrase mercy
                ui.horizontal(|ui| {
                    ui.label("New phrase:");
                    ui.text_edit_singleline(&mut ui_state.new_phrase);
                    if ui.button("Add").clicked() && !ui_state.new_phrase.is_empty() {
                        config.whitelist_phrases.insert(ui_state.new_phrase.clone());
                        ui_state.new_phrase.clear();
                    }
                });
            });
    }
}

pub struct WhitelistUIPlugin;

impl Plugin for WhitelistUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WhitelistUIState::default())
            .add_systems(Update, whitelist_ui_system);
    }
}
