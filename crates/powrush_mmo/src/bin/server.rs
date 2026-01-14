//! crates/powrush_mmo/src/bin/server.rs — Server authoritative eternal abundance universe
//! Lightyear server plugin + shared systems mercy
//! Hosts multiplayer shared thriving

use bevy::prelude::*;
use lightyear::prelude::server::*;
use powrush_mmo::protocol::MyProtocol;
use powrush_mmo::shared::SharedPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LightyearServerPlugin::<MyProtocol>::new(ServerConfig {
            transport: ServerTransport::UdpSocket("0.0.0.0:5000".parse().unwrap()),
            ..default()
        }))
        .add_plugins(SharedPlugin)
        .run();
}
