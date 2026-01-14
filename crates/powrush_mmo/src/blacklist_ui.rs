//! crates/powrush_mmo/src/blacklist_ui.rs
//! Dynamic blacklist management UI mercy eternal supreme immaculate
//! EGUI runtime entry add/remove for MercyShield philotic mercy

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::mercy_shield::MercyShieldConfig;

#[derive(Resource, Default)]
pub struct BlacklistUIState {
    pub show_window: bool,
    pub new_entry: String,
}

pub fn blacklist_ui_system(
    mut contexts: EguiContexts,
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: Local<BlacklistUIState>,
    mut config: ResMut<MercyShieldConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::B) {
        ui_state.show_window = !ui_state.show_window;
    }

    if ui_state.show_window {
        egui::Window::new("MercyShield Blacklist — Blocked Entries Mercy Eternal")
            .resizable(true)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Blocked Entries (auto-block mercy)");

                // List current entries mercy
                let mut to_remove = None;
                for (i, entry) in config.blacklist.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(entry);
                        if ui.button("Remove").clicked() {
                            to_remove = Some(entry.clone());
                        }
                    });
                }

                if let Some(entry) = to_remove {
                    config.blacklist.remove(&entry);
                }

                // Add new entry mercy
                ui.horizontal(|ui| {
                    ui.label("New entry (phrase/ID):");
                    ui.text_edit_singleline(&mut ui_state.new_entry);
                    if ui.button("Add").clicked() && !ui_state.new_entry.is_empty() {
                        config.blacklist.insert(ui_state.new_entry.clone());
                        ui_state.new_entry.clear();
                    }
                });
            });
    }
}

pub struct BlacklistUIPlugin;

impl Plugin for BlacklistUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlacklistUIState::default())
            .add_systems(Update, blacklist_ui_system);
    }
}
