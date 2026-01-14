use bevy::prelude::*;
use crate::main::ReverbZone;

pub fn spawn_reverb_zones(mut commands: Commands) {
    // Example cave reverb zone mercy
    commands.spawn((
        Transform::from_xyz(100.0, 0.0, 100.0),
        GlobalTransform::default(),
        Visibility::Visible,
        ReverbZone {
            reverb_time: 3.0,
            damping: 0.8,
            intensity: 0.9,
        },
    ));

    // Open field mercy
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        ReverbZone {
            reverb_time: 0.5,
            damping: 0.3,
            intensity: 0.2,
        },
    ));
}
