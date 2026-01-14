use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, DefaultChannel};
use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

/// Network Messages — Position + Velocity Sync + Spawn + Audio Eternal
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum NetworkMessage {
    EntitySpawn {
        net_id: u64,
        entity_type: EntityType,
        position: Vec3,
        velocity: Vec3,
    },
    PlayerUpdate {
        net_id: u64,
        position: Vec3,
        velocity: Vec3,
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
pub struct NetworkId(pub u64);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource, Default)]
pub struct NextNetworkId(pub u64);

/// Multiplayer Replication Plugin — Prediction + Sync Mercy Eternal
pub struct MultiplayerReplicationPlugin;

impl Plugin for MultiplayerReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextNetworkId::default())
            .add_systems(Update, (
                server_spawn_entities,
                server_broadcast_updates,
                client_send_update,
                client_receive_messages,
            ));
    }
}

fn server_spawn_entities(
    mut commands: Commands,
    mut next_id: ResMut<NextNetworkId>,
    mut server: ResMut<RenetServer>,
    added_players: Query<(Entity, &Transform), Added<Player>>,
) {
    for (entity, transform) in &added_players {
        let net_id = {
            let id = next_id.0;
            next_id.0 += 1;
            id
        };
        commands.entity(entity).insert((NetworkId(net_id), Velocity(Vec3::ZERO)));

        let msg = NetworkMessage::EntitySpawn {
            net_id,
            entity_type: EntityType::Player,
            position: transform.translation,
            velocity: Vec3::ZERO,
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, payload);
    }
}

fn server_broadcast_updates(
    mut server: ResMut<RenetServer>,
    query: Query<(&NetworkId, &Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (net_id, transform, velocity) in &query {
        let msg = NetworkMessage::PlayerUpdate {
            net_id: net_id.0,
            position: transform.translation,
            velocity: velocity.0,
            timestamp: time.elapsed_seconds_f64(),
        };
        let payload = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, payload);
    }
}

fn client_send_update(
    mut client: ResMut<RenetClient>,
    query: Query<(&NetworkId, &Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((net_id, transform, velocity)) = query.get_single() {
        let msg = NetworkMessage::PlayerUpdate {
            net_id: net_id.0,
            position: transform.translation,
            velocity: velocity.0,
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
    mut query: Query<(&NetworkId, &mut Transform, &mut Velocity)>,
    audio: Res<Audio>,
) {
    // Reliable spawns
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            if let NetworkMessage::EntitySpawn { net_id, entity_type, position, velocity } = msg {
                let bundle = match entity_type {
                    EntityType::Player => PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                    EntityType::Resource => PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                        material: materials.add(Color::rgb(1.0, 0.8, 0.2).into()),
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                };
                commands.spawn((bundle, NetworkId(net_id), Velocity(velocity)));
            }
        }
    }

    // Unreliable updates + audio
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        if let Ok((msg, _)) = bincode::decode_from_slice::<NetworkMessage, _>(&message, bincode::config::standard()) {
            match msg {
                NetworkMessage::PlayerUpdate { net_id, position, velocity, .. } => {
                    for (id, mut transform, mut vel) in &mut query {
                        if id.0 == net_id {
                            // Reconciliation for local player (if is local)
                            let diff = (transform.translation - position).length();
                            if diff > 0.5 {
                                transform.translation = position;  // Snap correction mercy
                            }
                            vel.0 = velocity;
                        } else {
                            // Dead reckoning for remote
                            transform.translation = position;
                            vel.0 = velocity;
                        }
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
