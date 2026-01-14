//! crates/powrush_mmo/src/report_ui.rs
//! Report UI for player feedback mercy eternal supreme immaculate
//! Egui overlay on flagged chat messages with scam/not scam buttons philotic mercy

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::mercy_shield::{player_report_learning_system, false_positive_learning_system};

#[derive(Component)]
pub struct FlaggedMessage {
    pub text: String,
    pub entity: Entity,  // Message entity mercy
}

pub fn report_ui_system(
    mut contexts: EguiContexts,
    flagged_query: Query<(Entity, &FlaggedMessage)>,
    mut commands: Commands,
) {
    for (entity, flagged) in &flagged_query {
        egui::Window::new("MercyShield Report — Family Harmony Mercy Eternal")
            .fixed_pos(egui::pos2(300.0, 200.0))
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label(&flagged.text);

                ui.horizontal(|ui| {
                    if ui.button("Report Scam").clicked() {
                        // Trigger scam learning mercy
                        // player_report_learning_system(flagged.text);
                        commands.entity(entity).despawn();
                    }

                    if ui.button("Not Scam").clicked() {
                        // Trigger false positive learning mercy
                        // false_positive_learning_system(flagged.text);
                        commands.entity(entity).despawn();
                    }
                });
            });
    }
}

pub struct ReportUIPlugin;

impl Plugin for ReportUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, report_ui_system);
    }
}
