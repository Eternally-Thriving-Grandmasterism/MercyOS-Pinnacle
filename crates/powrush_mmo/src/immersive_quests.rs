//! crates/powrush_mmo/src/immersive_quests.rs
//! Immersive quest system — crayon UI, playful narration, joy particles mercy eternal supreme immaculate
//! Child Wonder Mode enhanced with Rugrats-style adventure philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use egui::{RichText, Color32};

#[derive(Resource)]
pub struct QuestState {
    pub active_quest: Option<QuestType>,
    pub progress: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    FindDidi,
    TameReptar,
    ShareFood,
}

pub fn immersive_quest_ui(
    mut contexts: EguiContexts,
    quest_state: Res<QuestState>,
    wonder_mode: Query<&ChildWonderMode>,
) {
    if wonder_mode.get_single().is_ok() && quest_state.active_quest.is_some() {
        egui::Window::new("Baby Adventure Log — Crayon Mercy Eternal")
            .resizable(false)
            .fixed_pos(egui::pos2(100.0, 100.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label(RichText::new("Big Adventure Time!").size(32.0).color(Color32::from_rgb(255, 100, 200)));

                match quest_state.active_quest.unwrap() {
                    QuestType::FindDidi => {
                        ui.label(RichText::new("Find Mommy Didi! She's hiding somewhere big!").size(24.0).color(Color32::LIGHT_BLUE));
                    }
                    QuestType::TameReptar => {
                        ui.label(RichText::new("Be friends with giant Reptar! Give him yummy food!").size(24.0).color(Color32::GREEN));
                    }
                    QuestType::ShareFood => {
                        ui.label(RichText::new("Share yummy food with friends! Everyone happy!").size(24.0).color(Color32::YELLOW));
                    }
                }

                ui.label(RichText::new(format!("Progress: {:.0}%", quest_state.progress * 100.0)).size(20.0));
            });
    }
}

pub fn quest_joy_particles(
    quest_state: Res<QuestState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if quest_state.is_changed() && quest_state.progress >= 1.0 {
        let player_pos = player_query.single().translation;

        // Massive joy sparkle burst mercy eternal
        for _ in 0..50 {
            let offset = Vec3::new(
                rand::thread_rng().gen_range(-5.0..5.0),
                rand::thread_rng().gen_range(0.0..10.0),
                rand::thread_rng().gen_range(-5.0..5.0),
            );

            let color = Color::hsl(rand::thread_rng().gen_range(0.0..360.0), 1.0, 0.7);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.3, ..default() })),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive: color * 10.0,
                        ..default()
                    }),
                    transform: Transform::from_translation(player_pos + offset),
                    visibility: Visibility::Visible,
                    ..default()
                },
                ImaginationSparkle,
                Lifetime(2.0),
            ));
        }

        // Joyful completion sound mercy
        // audio.play(joy_sound);
    }
}

pub struct ImmersiveQuestPlugin;

impl Plugin for ImmersiveQuestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(QuestState { active_quest: None, progress: 0.0 })
            .add_systems(Update, (immersive_quest_ui, quest_joy_particles));
    }
}
