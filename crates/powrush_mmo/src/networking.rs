use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel, ClientId};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

/// Audio Event Types — Philotic Multiplayer Sync Eternal
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

/// Multiplayer Audio Sync Plugin — Server/Client Mercy Eternal
pub struct MultiplayerAudioPlugin;

impl Plugin for MultiplayerAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            server_broadcast_audio_events,
            client_receive_audio_events,
        ));
    }
}

fn server_broadcast_audio_events(
    mut server: ResMut<RenetServer>,
    query: Query<&Transform, With<Player>>,
) {
    // Example: Broadcast on joy pulse — real trigger from emotional system
    for transform in &query {
        let event = AudioEvent::EmotionalChime {
            position: transform.translation,
            base_freq: 440.0,
            joy_level: 8.0,
            duration: 2.0,
        };
        let payload = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, payload);
    }
}

fn client_receive_audio_events(
    mut client: ResMut<RenetClient>,
    audio: Res<Audio>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok((event, _)) = bincode::decode_from_slice::<AudioEvent, _>(&message, bincode::config::standard()) {
            match event {
                AudioEvent::EmotionalChime { position, base_freq, joy_level, duration } => {
                    // Play locally with spatial mercy
                    let chime = ultimate_fm_synthesis(base_freq, joy_level, duration);
                    audio.play(chime)
                        .with_volume(0.5 + joy_level * 0.4)
                        .spatial(true)
                        .with_position(position);
                }
                AudioEvent::GranularAmbient { position, joy_level } => {
                    spawn_pure_procedural_granular_ambient(&audio, joy_level, position);
                }
            }
        }
    }
}
