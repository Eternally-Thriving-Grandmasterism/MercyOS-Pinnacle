//! powrush_mmo/bin/server.rs — Server authoritative universe
//! Hosts eternal shared abundance | Lightyear supreme

use bevy::prelude::*;
use lightyear::prelude::server::*;
use powrush_mmo::protocol::*;
use powrush_mmo::shared::SharedPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LightyearPlugin::<MyProtocol>::new(ServerConfig {
            transport: ServerTransport::UdpSocket("0.0.0.0:5000".parse().unwrap()),
            ..default()
        }))
        .add_plugins(SharedPlugin)
        .run();
}
