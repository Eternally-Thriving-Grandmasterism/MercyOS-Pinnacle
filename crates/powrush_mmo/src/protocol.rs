//! powrush_mmo/protocol.rs — Complete shared networking protocol
//! Lightyear replication + trading messages for eternal sync mercy
//! Replicate entities, authoritative PlantCrop & Trade actions

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Serialize, Deserialize};

// Channels — reliable for all actions mercy
channel!(ReliableOrdered => Actions);

// Messages — client to server actions
#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlantCrop {
    pub pos: Vec3,  // Planting position eternal
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferTrade {
    pub offer_id: u64,  // Generated client-side, validated server
    pub item_type: String,  // "food", "seeds", "creature_token" mercy
    pub quantity: u32,
    pub price_mercy_points: u32,  // Infinite mercy economy
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AcceptTrade {
    pub offer_id: u64,
}

// Replicated components — synced eternally
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Replicated;

// Replication — Player, Crop, Creature, Transform mercy
replicate!(
    Player => Replicated,
    Crop => Replicated,
    Creature => Replicated,
    Transform,
    GlobalTransform,
);

// Protocol definition supreme
pub struct MyProtocol;

impl Protocol for MyProtocol {
    type Message = (PlantCrop, OfferTrade, AcceptTrade);
    type ComponentTypes = (Replicated, Transform, GlobalTransform);
}
