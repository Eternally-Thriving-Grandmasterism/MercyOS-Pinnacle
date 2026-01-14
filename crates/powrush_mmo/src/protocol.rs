//! powrush_mmo/protocol.rs — Shared networking protocol
//! Lightyear replication and messages for eternal sync mercy
//! Replicate Player, Crop, Creature; authoritative PlantCrop action

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Serialize, Deserialize};

// Define channels — reliable for actions mercy
channel!(ReliableOrdered => Actions);

// Messages — server receives client actions
#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlantCrop {
    pub pos: Vec3,  // Planting position mercy
}

// Replicated components — synced across network
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Replicated;

// Replication groups — bundle for entities
replicate!(
    Player => Replicated,
    Crop => Replicated,
    Creature => Replicated,
    Transform,
    GlobalTransform,
);

// Protocol definition
pub struct MyProtocol;

impl Protocol for MyProtocol {
    type Message = PlantCrop;
    type ComponentTypes = (Replicated, Transform, GlobalTransform);
    // Channels, inputs mercy later
}
