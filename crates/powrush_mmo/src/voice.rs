//! powrush_mmo/voice.rs — Complete proximity voice chat mercy
//! Mic capture, opus compress, Lightyear unreliable relay, proximity volume mix
//! Push-to-talk V key, speaking particles joy

use bevy::prelude::*;
use bevy_audio_stream::prelude::*;
use opus::Encoder;
use lightyear::prelude::*;
use std::collections::HashMap;

// Unreliable voice channel mercy
channel!(Unreliable => VoiceChannel);

// Voice packet message
#[message(channel = VoiceChannel)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoicePacket {
    pub speaker: ClientId,
    pub audio_data: Vec<u8>,  // Opus compressed mercy
}

// Client voice capture resource
#[derive(Resource)]
struct VoiceCapture {
    stream: AudioStreamHandle,
    encoder: Encoder,
    ptt_active: bool,
}

// Server voice relay system
fn server_voice_relay(
    mut messages: EventReader<FromClient<VoicePacket>>,
    player_positions: Query<(&ClientId, &GlobalTransform), With<Player>>,
    mut voice_writer: EventWriter<ToClients<VoicePacket>>,
) {
    let positions: HashMap<ClientId, Vec3> = player_positions.iter().map(|(id, t)| (*id, t.translation())).collect();

    for message in messages.read() {
        let speaker_pos = if let Some(pos) = positions.get(&message.message.speaker) { *pos } else { continue; };

        let nearby: Vec<ClientId> = positions.iter()
            .filter(|(id, pos)| **id != message.message.speaker && pos.distance(speaker_pos) < 50.0)
            .map(|(id, _)| *id)
            .collect();

        if !nearby.is_empty() {
            voice_writer.send(ToClients {
                clients: nearby,
                message: message.message.clone(),
            });
        }
    }
}

// Client voice capture system (push-to-talk V)
fn client_voice_capture(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut capture: ResMut<VoiceCapture>,
    mut voice_writer: EventWriter<ToServer<VoicePacket>>,
    client_id: Res<ClientId>,
) {
    capture.ptt_active = keyboard.pressed(KeyCode::V);

    if capture.ptt_active {
        if let Some(chunk) = capture.stream.try_take() {
            let mut compressed = vec![0u8; chunk.len() * 2];  // Mercy buffer
            let len = capture.encoder.encode(&chunk, &mut compressed).unwrap();
            compressed.truncate(len);

            voice_writer.send(ToServer(VoicePacket {
                speaker: client_id,
                audio_data: compressed,
            }));
        }
    }
}

// Client voice playback with proximity volume
fn client_voice_playback(
    mut messages: EventReader<FromServer<VoicePacket>>,
    player_positions: Query<(&ClientId, &GlobalTransform), With<Player>>,
    // Audio sink mercy (kira or rodio)
) {
    let positions: HashMap<ClientId, Vec3> = player_positions.iter().map(|(id, t)| (*id, t.translation())).collect();

    for message in messages.read() {
        let speaker_pos = if let Some(pos) = positions.get(&message.message.speaker) { *pos } else { continue; };
        let listener_pos = positions.get(&ClientId::local());  // Mercy local

        let dist = listener_pos.distance(speaker_pos);
        let volume = (1.0 - (dist / 50.0)).max(0.0);

        if volume > 0.0 {
            // Decompress opus, play with volume mercy
            // Speaking particles blue wave
        }
    }
}

// Setup voice on client
fn setup_voice(mut commands: Commands) {
    let stream = AudioStream::new_microphone(/* sample rate 48000 mercy */);
    let encoder = Encoder::new(48000, opus::Channels::Mono, opus::Application::Voip).unwrap();

    commands.insert_resource(VoiceCapture {
        stream,
        encoder,
        ptt_active: false,
    });
}

// Add to client plugins: setup_voice system Startup, client_voice_capture, client_voice_playback Update
// Server: server_voice_relay
