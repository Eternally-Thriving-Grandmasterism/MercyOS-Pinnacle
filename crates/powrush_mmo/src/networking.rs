use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel, ClientId};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

/// Network Messages — Entity Replication + Position + Audio Eternal
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum NetworkMessage {
    EntitySpawn {
        net_id: u64,
        entity_type: EntityType,
        position: Vec3,
    },
    PlayerPosition {
        net_id: u64,
        position: Vec3,
        timestamp: f64,
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
pub struct NetworkId(u64);

/// Multiplayer Replication Plugin — Entity + Position + Audio Mercy Eternal
pub struct MultiplayerReplicationPlugin;

impl Plugin for MultiplayerReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            server_spawn_entities,
            server_broadcast_positions,
            server_broadcast_audio_events,
            client_receive_messages,
            client_send_position,
        ));
    }
}

fn server_spawn_entities(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), (With<Player>, Added<Player>)>,
) {
    static mut NEXT_ID: u64 = 1;
    for (entity, transform) in &query {
        let net_id = unsafe { NEXT_ID += 1; NEXT_ID - 1 };
        commands.entity(entity).insert(NetworkId(net_id));

        let msg = NetworkMessage::EntitySpawn {
            net_id,
            entity_type: EntityType::Player,
            position: transform.translation,
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, payload);
    }
}

fn server_broadcast_positions(
    mut server: ResMut<RenetServer>,
    query: Query<(&NetworkId, &Transform)>,
    time: Res<Time>,
) {
    for (net_id, transform) in &query {
        let msg = NetworkMessage::PlayerPosition {
            net_id: net_id.0,
            position: transform.translation,
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_send_position(
    mut client: ResMut<RenetClient>,
    query: Query<(&NetworkId, &Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((net_id, transform)) = query.get_single() {
        let msg = NetworkMessage::PlayerPosition {
            net_id: net_id.0,
            position: transform.translation,
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        client.send_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_receive_messages(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Transform, With<NetworkId>>,
    audio: Res<Audio>,
) {
    // Reliable channel for spawns
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            match msg {
                NetworkMessage::EntitySpawn { net_id, entity_type, position } => {
                    let entity = match entity_type {
                        EntityType::Player => {
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                                    material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
                                    transform: Transform::from_translation(position),
                                    ..default()
                                },
                                NetworkId(net_id),
                            )).id()
                        }
                        EntityType::Resource => {
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                                    material: materials.add(Color::rgb(1.0, 0.8, 0.2).into()),
                                    transform: Transform::from_translation(position),
                                    ..default()
                                },
                                NetworkId(net_id),
                            )).id()
                        }
                    };
                }
                _ => {}
            }
        }
    }

    // Unreliable for positions + audio
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            match msg {
                NetworkMessage::PlayerPosition { net_id, position, .. } => {
                    if let Ok(mut transform) = query.get_mut(Entity::from_bits(net_id)) {
                        transform.translation = position;
                    }
                }
                NetworkMessage::AudioEvent(event) => {
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
