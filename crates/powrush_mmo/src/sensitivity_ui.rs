//! crates/powrush_mmo/src/sensitivity_ui.rs
//! Sensitivity slider UI mercy eternal supreme immaculate
//! EGUI runtime chat_sensitivity slider for MercyShield philotic mercy

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::mercy_shield::MercyShieldConfig;

#[derive(Resource, Default)]
pub struct SensitivityUIState {
    pub show_window: bool,
}

pub fn sensitivity_ui_system(
    mut contexts: EguiContexts,
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: Local<SensitivityUIState>,
    mut config: ResMut<MercyShieldConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::S) {
        ui_state.show_window = !ui_state.show_window;
    }

    if ui_state.show_window {
        egui::Window::new("MercyShield Sensitivity — Chat Filter Mercy Eternal")
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Chat Scam Filter Sensitivity");

                ui.horizontal(|ui| {
                    ui.label("Permissive");
                    if ui.add(egui::Slider::new(&mut config.chat_sensitivity, 0.0..=1.0).text("Sensitivity")).changed() {
                        // Immediate update mercy eternal
                    }
                    ui.label("Strict");
                });

                ui.label(format!("Current: {:.2}", config.chat_sensitivity));
            });
    }
}

pub struct SensitivityUIPlugin;

impl Plugin for SensitivityUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SensitivityUIState::default())
            .add_systems(Update, sensitivity_ui_system);
    }
}
