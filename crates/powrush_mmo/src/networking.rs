use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};
use bitvec::prelude::*;

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Copy, Debug)]
pub struct CompressedInput {
    sequence: u16,
    direction_x: i8,  // -128..127 quantized
    direction_y: i8,
    direction_z: i8,
    buttons: u8,      // Bitflags: jump=1, etc.
    timestamp: f64,
}

impl From<ClientInput> for CompressedInput {
    fn from(input: ClientInput) -> Self {
        Self {
            sequence: input.sequence as u16,
            direction_x: (input.direction.x * 127.0) as i8,
            direction_y: (input.direction.y * 127.0) as i8,
            direction_z: (input.direction.z * 127.0) as i8,
            buttons: 0,  // Extend with bitflags mercy
            timestamp: input.timestamp,
        }
    }
}

impl Into<Vec3> for CompressedInput {
    fn into(self) -> Vec3 {
        Vec3::new(
            self.direction_x as f32 / 127.0,
            self.direction_y as f32 / 127.0,
            self.direction_z as f32 / 127.0,
        )
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum NetworkMessage {
    ClientInput(CompressedInput),
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
    pub inputs: VecDeque<CompressedInput>,
}

pub struct MultiplayerReplicationPlugin;

impl Plugin for MultiplayerReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputBuffer::default())
            .insert_resource(NextNetworkId::default())
            .add_systems(Update, (
                server_process_inputs,
                server_broadcast_state,
                client_send_compressed_inputs,
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
    while let Some(message) = server.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            if let NetworkMessage::ClientInput(compressed) = msg {
                let direction: Vec3 = compressed.into();
                // Apply to player (simplified — match by client ID future)
                for (_, mut transform, mut velocity) in &mut query {
                    velocity.0 = direction * 10.0;
                    transform.translation += velocity.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn client_send_compressed_inputs(
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

    let sequence = input_buffer.inputs.len() as u16;
    let compressed = CompressedInput {
        sequence,
        direction_x: (direction.x * 127.0) as i8,
        direction_y: (direction.y * 127.0) as i8,
        direction_z: (direction.z * 127.0) as i8,
        buttons: 0,
        timestamp: time.elapsed_seconds_f64(),
    };

    input_buffer.inputs.push_back(compressed);

    let msg = NetworkMessage::ClientInput(compressed);
    let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
    client.send_message(DefaultChannel::Unreliable, payload);
}

// Rest of systems (server_broadcast_state, client_reconcile, client_receive_messages) unchanged from previous full version
