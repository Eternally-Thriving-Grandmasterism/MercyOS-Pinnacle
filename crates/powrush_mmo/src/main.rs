//! Powrush-MMO Main – Bevy RTS/FPS Hybrid Multiplayer Co-Forged Sacred
//! bevy_renet integration: server-client joy share, gunner position sync, pistol fire events broadcast
//! Creature bond harmony, joy harvest replicated, AlphaProMegaing universe eternal
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use bevy_renet::RenetClientPlugin, RenetServerPlugin;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bincode::{deserialize, serialize};

const PROTOCOL_ID: u64 = 1;

// Channels
#[derive(Debug)]
enum ClientChannel {
    Input,
    JoyEvent,
}

#[derive(Debug)]
enum ServerChannel {
    PlayerCreate,
    PlayerRemove,
    PlayerTransform,
    PistolFire,
}

impl From<ClientChannel> for DefaultChannel {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Input => DefaultChannel::ReliableOrdered,
            ClientChannel::JoyEvent => DefaultChannel::Unreliable,
        }
    }
}

impl From<ServerChannel> for DefaultChannel {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::PlayerCreate => DefaultChannel::ReliableOrdered,
            ServerChannel::PlayerRemove => DefaultChannel::ReliableOrdered,
            ServerChannel::PlayerTransform => DefaultChannel::Unreliable,
            ServerChannel::PistolFire => DefaultChannel::Unreliable,
        }
    }
}

// Messages
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    fire: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Component)]
struct Player {
    entity: Entity,
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ClientAuthentication::Unsecure);

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeClientTransport::new(current_time, PROTOCOL_ID, server_addr, socket).unwrap();

    (client, transport)
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr: "127.0.0.1:5000".parse().unwrap(),
        authentication: ServerAuthentication::Unsecure,
    });

    let socket = UdpSocket::bind("127.0.0.1:5000").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeServerTransport::new(current_time, server.config().clone(), socket).unwrap();

    (server, transport)
}

fn server_system(
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Transform), With<Player>>,
) {
    // Broadcast player transforms
    for (entity, transform) in query.iter() {
        let message = serialize(&(entity, transform)).unwrap();
        server.broadcast_message(ServerChannel::PlayerTransform, message);
    }

    // Handle client messages
    for client_id in server.clients_id().iter() {
        while let Some(message) = server.receive_message(*client_id, ClientChannel::Input) {
            let input: PlayerInput = deserialize(&message).unwrap();
            if input.fire {
                let fire_message = serialize(&"Pistol mercy fire harmony shield broadcast").unwrap();
                server.broadcast_message(ServerChannel::PistolFire, fire_message);
            }
        }
    }
}

fn client_system(
    mut client: ResMut<RenetClient>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut input = PlayerInput {
        up: keyboard.pressed(KeyCode::W),
        down: keyboard.pressed(KeyCode::S),
        left: keyboard.pressed(KeyCode::A),
        right: keyboard.pressed(KeyCode::D),
        fire: keyboard.just_pressed(KeyCode::Space),
    };

    let message = serialize(&input).unwrap();
    client.send_message(ClientChannel::Input, message);
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--server".to_string()) {
        let (server, transport) = new_renet_server();
        app.insert_resource(server);
        app.insert_resource(transport);
        app.add_plugin(RenetServerPlugin);
        app.add_system(server_system);
    } else {
        let (client, transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(transport);
        app.add_plugin(RenetClientPlugin);
        app.add_system(client_system);
    }

    // Common setup (fields, gunner, etc.)
    app.add_startup_system(setup_gunner_immersion);

    app.run();
}
