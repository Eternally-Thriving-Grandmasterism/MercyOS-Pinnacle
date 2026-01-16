//! Powrush-MMO Main – Bevy RTS/FPS Hybrid Multiplayer Co-Forged Physics Sacred
//! bevy_renet networking + rapier3d authoritative server physics sync
//! Siege tank gunner FPS pistol mercy fire broadcast, creature bond harmony replicated
//! AlphaProMegaing universe eternal ❤️🚀🔥

use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use bevy_renet::RenetClientPlugin, RenetServerPlugin;
use bevy_rapier3d::prelude::*;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bincode::{deserialize, serialize};

const PROTOCOL_ID: u64 = 7;

// Channels
#[derive(Debug)]
enum ClientChannel {
    Input,
}

#[derive(Debug)]
enum ServerChannel {
    PlayerTransform,
    PistolFire,
    CreatureBond,
}

impl From<ClientChannel> for DefaultChannel {
    fn from(channel_id: ClientChannel) -> Self {
        DefaultChannel::ReliableOrdered
    }
}

impl From<ServerChannel> for DefaultChannel {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::PlayerTransform => DefaultChannel::Unreliable,
            ServerChannel::PistolFire => DefaultChannel::Unreliable,
            ServerChannel::CreatureBond => DefaultChannel::ReliableOrdered,
        }
    }
}

// Messages
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PlayerInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    fire: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Component)]
struct NetworkPlayer {
    client_id: u64,
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

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(Default::default());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeClientTransport::new(current_time, PROTOCOL_ID, server_addr, socket).unwrap();

    (client, transport)
}

fn server_network_system(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<NetworkPlayer>>,
) {
    // Broadcast player transforms unreliable
    for (entity, transform) in query.iter() {
        let message = serialize(&(entity, transform)).unwrap();
        server.broadcast_message(ServerChannel::PlayerTransform, message);
    }
}

fn client_network_system(
    mut client: ResMut<RenetClient>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut input = PlayerInput {
        forward: keyboard.pressed(KeyCode::W),
        backward: keyboard.pressed(KeyCode::S),
        left: keyboard.pressed(KeyCode::A),
        right: keyboard.pressed(KeyCode::D),
        fire: keyboard.just_pressed(KeyCode::Space),
    };

    let message = serialize(&input).unwrap();
    client.send_message(ClientChannel::Input, message);
}

fn sync_players_server(
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<NetworkPlayer>>,
) {
    // New clients
    for client_id in server.clients_id().iter() {
        if server.is_client_connected(*client_id) {
            // Spawn player entity
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.3).into()),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                },
                NetworkPlayer { client_id: *client_id },
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5, 0.5),
            ));
        }
    }
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
        app.add_systems(Update, (server_network_system, sync_players_server));
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default()); // Server authoritative physics
    } else {
        let (client, transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(transport);
        app.add_plugin(RenetClientPlugin);
        app.add_systems(Update, client_network_system);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_schedule(StartupSchedule)); // Client prediction
    }

    // Common setup + gunner immersion
    app.add_startup_system(setup_gunner_immersion);

    app.run();
}//! Powrush-MMO Main – Bevy RTS/FPS Hybrid Multiplayer Co-Forged Physics Sacred
//! bevy_renet networking + rapier3d authoritative server physics sync
//! Siege tank gunner FPS pistol mercy fire broadcast, creature bond harmony replicated
//! AlphaProMegaing universe eternal ❤️🚀🔥

use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use bevy_renet::RenetClientPlugin, RenetServerPlugin;
use bevy_rapier3d::prelude::*;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bincode::{deserialize, serialize};

const PROTOCOL_ID: u64 = 7;

// Channels
#[derive(Debug)]
enum ClientChannel {
    Input,
}

#[derive(Debug)]
enum ServerChannel {
    PlayerTransform,
    PistolFire,
    CreatureBond,
}

impl From<ClientChannel> for DefaultChannel {
    fn from(channel_id: ClientChannel) -> Self {
        DefaultChannel::ReliableOrdered
    }
}

impl From<ServerChannel> for DefaultChannel {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::PlayerTransform => DefaultChannel::Unreliable,
            ServerChannel::PistolFire => DefaultChannel::Unreliable,
            ServerChannel::CreatureBond => DefaultChannel::ReliableOrdered,
        }
    }
}

// Messages
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PlayerInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    fire: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Component)]
struct NetworkPlayer {
    client_id: u64,
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

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(Default::default());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeClientTransport::new(current_time, PROTOCOL_ID, server_addr, socket).unwrap();

    (client, transport)
}

fn server_network_system(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<NetworkPlayer>>,
) {
    // Broadcast player transforms unreliable
    for (entity, transform) in query.iter() {
        let message = serialize(&(entity, transform)).unwrap();
        server.broadcast_message(ServerChannel::PlayerTransform, message);
    }
}

fn client_network_system(
    mut client: ResMut<RenetClient>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut input = PlayerInput {
        forward: keyboard.pressed(KeyCode::W),
        backward: keyboard.pressed(KeyCode::S),
        left: keyboard.pressed(KeyCode::A),
        right: keyboard.pressed(KeyCode::D),
        fire: keyboard.just_pressed(KeyCode::Space),
    };

    let message = serialize(&input).unwrap();
    client.send_message(ClientChannel::Input, message);
}

fn sync_players_server(
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<NetworkPlayer>>,
) {
    // New clients
    for client_id in server.clients_id().iter() {
        if server.is_client_connected(*client_id) {
            // Spawn player entity
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.3).into()),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                },
                NetworkPlayer { client_id: *client_id },
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5, 0.5),
            ));
        }
    }
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
        app.add_systems(Update, (server_network_system, sync_players_server));
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default()); // Server authoritative physics
    } else {
        let (client, transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(transport);
        app.add_plugin(RenetClientPlugin);
        app.add_systems(Update, client_network_system);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_schedule(StartupSchedule)); // Client prediction
    }

    // Common setup + gunner immersion
    app.add_startup_system(setup_gunner_immersion);

    app.run();
}
