use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum NetworkMessage {
    ClientInput {
        sequence: u32,
        direction: Vec3,
        timestamp: f64,
    },
    ServerState {
        net_id: u64,
        position: Vec3,
        velocity: Vec3,
        last_processed_sequence: u32,
        timestamp: f64,
    },
    EntitySpawn {
        net_id: u64,
        entity_type: EntityType,
        position: Vec3,
    },
    AudioEvent(AudioEvent),
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum EntityType {
    Player,
    Resource,
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

#[derive(Component)]
pub struct NetworkId(pub u64);

#[derive(Component)]
pub struct Predicted;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub inputs: VecDeque<ClientInput>,
}

#[derive(Clone, Copy)]
pub struct ClientInput {
    pub sequence: u32,
    pub direction: Vec3,
    pub timestamp: f64,
}

pub struct MultiplayerReplicationPlugin;

impl Plugin for MultiplayerReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputBuffer::default())
            .insert_resource(NextNetworkId::default())
            .add_systems(Update, (
                server_process_inputs,
                server_broadcast_state,
                client_send_inputs,
                client_reconcile,
                client_receive_messages,
            ));
    }
}

#[derive(Resource, Default)]
pub struct NextNetworkId(pub u64);

fn server_process_inputs(
    mut server: ResMut<RenetServer>,
    mut query: Query<(&NetworkId, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    // Simple authoritative simulation — apply inputs
    while let Some(message) = server.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            if let NetworkMessage::ClientInput { sequence, direction, .. } = msg {
                // Find player and apply (simplified)
                for (_, mut transform, mut velocity) in &mut query {
                    velocity.0 = direction * 10.0;
                    transform.translation += velocity.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn server_broadcast_state(
    mut server: ResMut<RenetServer>,
    query: Query<(&NetworkId, &Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (net_id, transform, velocity) in &query {
        let msg = NetworkMessage::ServerState {
            net_id: net_id.0,
            position: transform.translation,
            velocity: velocity.0,
            last_processed_sequence: 0, // Placeholder — extend with per-client ack
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_send_inputs(
    mut client: ResMut<RenetClient>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) { direction.z -= 1.0; }
    if keyboard_input.pressed(KeyCode::S) { direction.z += 1.0; }
    if keyboard_input.pressed(KeyCode::A) { direction.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::D) { direction.x += 1.0; }
    if keyboard_input.pressed(KeyCode::Space) { direction.y += 1.0; }
    if keyboard_input.pressed(KeyCode::ShiftLeft) { direction.y -= 1.0; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let sequence = input_buffer.inputs.len() as u32;
    let input = ClientInput {
        sequence,
        direction,
        timestamp: time.elapsed_seconds_f64(),
    };

    input_buffer.inputs.push_back(input.clone());

    let msg = NetworkMessage::ClientInput {
        sequence,
        direction,
        timestamp: input.timestamp,
    };
    let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
    client.send_message(DefaultChannel::Unreliable, payload);
}

fn client_reconcile(
    mut client: ResMut<RenetClient>,
    mut query: Query<(&NetworkId, &mut Transform, &mut Velocity), With<Predicted>>,
    mut input_buffer: ResMut<InputBuffer>,
    time: Res<Time>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            if let NetworkMessage::ServerState { net_id, position, velocity, last_processed_sequence, .. } = msg {
                if let Ok((id, mut transform, mut vel)) = query.get_single_mut() {
                    if id.0 == net_id {
                        // Reconciliation mercy
                        let drift = (transform.translation - position).length();
                        if drift > 0.5 {
                            transform.translation = position;
                            vel.0 = velocity;

                            // Replay unacknowledged inputs
                            while let Some(input) = input_buffer.inputs.front() {
                                if input.sequence <= last_processed_sequence {
                                    input_buffer.inputs.pop_front();
                                } else {
                                    transform.translation += input.direction * 10.0 * time.delta_seconds();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn client_receive_messages(/* existing spawn + audio handling */) { /* unchanged + integrate with prediction */ }
