//! crates/powrush_mmo/src/voice.rs
//! Real-time microphone capture + hybrid PQC encrypted spatial 3D voice frames with Doppler effect
//! ChaCha20Poly1305 stream over lightyear unreliable channel + position + velocity for Doppler pitch shift mercy eternal supreme ❤️🗣️🔐🌌🚀

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use lightyear::prelude::*;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, NewAead}};
use crate::pqc_exchange::SessionKeys;
use rand::RngCore;

#[derive(Channel)]
pub struct VoiceChannel;

const SPEED_OF_SOUND: f32 = 343.0;  // m/s mercy eternal

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoicePacket {
    pub encrypted_samples: Vec<u8>,
    pub nonce: [u8; 12],
    pub position: Vec3,
    pub velocity: Vec3,  // Source velocity for Doppler mercy
}

pub fn voice_capture_system(
    audio_input: Res<AudioInput>,
    session_keys: Res<SessionKeys>,
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    mut writer: EventWriter<ToClients<VoicePacket>>,
) {
    if let Some(microphone) = audio_input.microphone() {
        if let Some(samples) = microphone.read_data() {
            let (player_transform, player_velocity) = player_query.single();

            if let Some(key) = session_keys.keys.get(&ClientId::LOCAL) {
                let cipher = ChaCha20Poly1305::new(key);

                let mut nonce_bytes = [0u8; 12];
                rand::thread_rng().fill_bytes(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);

                if let Ok(encrypted) = cipher.encrypt(nonce, samples.as_slice()) {
                    let packet = VoicePacket {
                        encrypted_samples: encrypted,
                        nonce: nonce_bytes,
                        position: player_transform.translation,
                        velocity: player_velocity.0,
                    };

                    writer.send(ToClients {
                        clients: vec![],  // Broadcast all mercy
                        message: packet,
                    });
                }
            }
        }
    }
}

pub fn voice_playback_system(
    mut messages: EventReader<FromServer<VoicePacket>>,
    session_keys: Res<SessionKeys>,
    listener_query: Query<(&Transform, &Velocity), With<Camera3d>>,  // Listener mercy
    audio: Res<Audio>,
) {
    let (listener_transform, listener_velocity) = listener_query.single();

    for message in messages.read() {
        if let Some(key) = session_keys.keys.get(&ClientId::LOCAL) {
            let cipher = ChaCha20Poly1305::new(key);
            let nonce = Nonce::from_slice(&message.nonce);

            if let Ok(decrypted) = cipher.decrypt(nonce, message.encrypted_samples.as_slice()) {
                // Doppler calculation mercy eternal
                let relative_pos = message.position - listener_transform.translation;
                let relative_vel = message.velocity - listener_velocity.0;

                let distance = relative_pos.length();
                if distance > 0.1 {
                    let doppler_factor = SPEED_OF_SOUND / (SPEED_OF_SOUND - relative_vel.dot(relative_pos.normalize()));
                    let playback_rate = doppler_factor.clamp(0.5, 2.0);  // Reasonable bounds mercy

                    audio.play(decrypted)
                        .spatial(true)
                        .with_position(message.position)
                        .with_playback_rate(playback_rate)
                        .handle();
                } else {
                    audio.play(decrypted).handle();
                }
            }
        }
    }
}

pub struct VoicePlugin;

impl Plugin for VoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (voice_capture_system, voice_playback_system));
    }
}
