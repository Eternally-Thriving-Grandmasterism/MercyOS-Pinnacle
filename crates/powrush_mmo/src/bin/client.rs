//! crates/powrush_mmo/src/bin/client.rs — Client predicted eternal thriving
//! Connects to localhost:5000 mercy, multiple instances for testing

use bevy::prelude::*;
use lightyear::prelude::client::*;
use powrush_mmo::protocol::MyProtocol;
use powrush_mmo::shared::SharedPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LightyearClientPlugin::<MyProtocol>::new(ClientConfig {
            transport: ClientTransport::UdpSocket("127.0.0.1:5000".parse().unwrap()),
            ..default()
        }))
        .add_plugins(SharedPlugin)
        .run();
}
