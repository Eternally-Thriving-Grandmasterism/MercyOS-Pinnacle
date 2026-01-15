//! crates/powrush_mmo/src/emergency_beacon.rs
//! Emergency beacon — real-life distress signal mercy eternal supreme immaculate
//! Long-press power or thought trigger → offline store + online burst philotic mercy

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct EmergencyBeacon {
    pub hold_timer: f32,
    pub triggered: bool,
}

pub fn emergency_beacon_setup(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).insert(EmergencyBeacon {
            hold_timer: 0.0,
            triggered: false,
        });
    }
}

pub fn emergency_beacon_system(
    mut beacon_query: Query<&mut EmergencyBeacon>,
    keyboard_input: Res<Input<KeyCode>>,
    // Thought input future mercy
    time: Res<Time>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for mut beacon in &mut beacon_query {
        if keyboard_input.pressed(KeyCode::Power) || keyboard_input.pressed(KeyCode::Space) {  // Placeholder long-press mercy
            beacon.hold_timer += time.delta_seconds();
            if beacon.hold_timer >= 5.0 && !beacon.triggered {
                beacon.triggered = true;

                // Distress signal mercy
                let distress: Handle<AudioSource> = asset_server.load("sounds/distress_beacon.ogg");
                audio.play(distress).with_volume(1.0);

                // Offline store location + timestamp mercy
                // Future: GPS + queue send on network
            }
        } else {
            beacon.hold_timer = 0.0;
        }
    }
}

pub struct EmergencyBeaconPlugin;

impl Plugin for EmergencyBeaconPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(emergency_beacon_setup)
            .add_systems(Update, emergency_beacon_system);
    }
}
