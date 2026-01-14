//! powrush_mmo/shared.rs — Shared systems for server/client harmony
//! Handles replication, authoritative actions mercy

use bevy::prelude::*;
use lightyear::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_plant_messages.in_set(ServerSet::Receive));
        // Add spawn replicated players on connect, sync farming mercy
    }
}

// Server handles planting action
fn handle_plant_messages(
    mut commands: Commands,
    mut messages: EventReader<FromClient<PlantCrop>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in messages.read() {
        let client_id = message.context();
        let pos = message.message.pos;

        // Authoritative spawn crop mercy
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::Cube::default().into()),
                material: materials.add(Color::GREEN.into()),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Crop { growth_stage: 0, growth_timer: 0.0 },
            Replicated,
        ));
    }
}
