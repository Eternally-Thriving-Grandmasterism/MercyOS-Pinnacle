use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel, ClientId};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

/// Network Messages — Player Position Sync + Audio Events Eternal
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum NetworkMessage {
    PlayerPosition {
        id: u64,
        position: Vec3,
        timestamp: f64,
    },
    PlayerSpawn {
        id: u64,
        position: Vec3,
    },
    AudioEvent(AudioEvent),  // From previous
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum AudioEvent {
    EmotionalChime {
        position: Vec3,
        base_freq: f32,
        joy_level: f32,
        duration: f32,
    },
    GranularAmbient {
        position: Vec3,
        joy_level: f32,
    },
}

/// Multiplayer Sync Plugin — Position + Audio Mercy Eternal
pub struct MultiplayerSyncPlugin;

impl Plugin for MultiplayerSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            server_broadcast_positions,
            server_broadcast_audio_events,
            client_receive_messages,
            client_send_position,
        ));
    }
}

fn server_broadcast_positions(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<Player>>,
    time: Res<Time>,
) {
    for (entity, transform) in &query {
        let id = entity.to_bits();
        let msg = NetworkMessage::PlayerPosition {
            id,
            position: transform.translation,
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_send_position(
    mut client: ResMut<RenetClient>,
    query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(transform) = query.get_single() {
        let msg = NetworkMessage::PlayerPosition {
            id: 0,  // Local placeholder
            position: transform.translation,
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        client.send_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_receive_messages(
    mut client: ResMut<RenetClient>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    audio: Res<Audio>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            match msg {
                NetworkMessage::PlayerPosition { id, position, .. } => {
                    // Simple snap or interpolate — future lerp mercy
                    if let Ok(mut transform) = query.get_mut(Entity::from_bits(id)) {
                        transform.translation = position;
                    }
                }
                NetworkMessage::AudioEvent(event) => {
                    // Existing audio handling
                    match event {
                        AudioEvent::EmotionalChime { position, base_freq, joy_level, duration } => {
                            let chime = ultimate_fm_synthesis(base_freq, joy_level, duration);
                            audio.play(chime).spatial(true).with_position(position);
                        }
                        AudioEvent::GranularAmbient { position, joy_level } => {
                            spawn_pure_procedural_granular_ambient(&audio, joy_level, position);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
