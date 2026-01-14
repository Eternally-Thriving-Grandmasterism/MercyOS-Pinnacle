//! powrush_mmo/bin/client.rs — Client predicted thriving
//! Connects to localhost:5000 | Multiple instances mercy

use bevy::prelude::*;
use lightyear::prelude::client::*;
use powrush_mmo::protocol::*;
use powrush_mmo::shared::SharedPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LightyearPlugin::<MyProtocol>::new(ClientConfig {
            transport: ClientTransport::UdpSocket("127.0.0.1:5000".parse().unwrap()),
            ..default()
        }))
        .add_plugins(SharedPlugin)
        .run();
}
