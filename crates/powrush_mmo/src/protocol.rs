//! crates/powrush_mmo/src/protocol.rs — Complete shared networking protocol
//! Lightyear replication + all messages for actions, trading, auctions mercy

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Serialize, Deserialize};

// Channels mercy
channel!(ReliableOrdered => Actions);
channel!(Unreliable => VoiceChannel);

// Messages mercy
#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlantCrop {
    pub pos: Vec3,
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferTrade {
    pub offer_id: u64,
    pub item_type: String,
    pub quantity: u32,
    pub price: u32,
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AcceptTrade {
    pub offer_id: u64,
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListAuction {
    pub auction_id: u64,
    pub item_type: String,
    pub quantity: u32,
    pub starting_price: u32,
    pub duration_seconds: f32,
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BidAuction {
    pub auction_id: u64,
    pub bid_amount: u32,
}

// Replication mercy
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Replicated;

replicate!(
    Player => Replicated,
    Crop => Replicated,
    Creature => Replicated,
    Transform,
    GlobalTransform,
);

// Protocol
pub struct MyProtocol;

impl Protocol for MyProtocol {
    type Message = (PlantCrop, OfferTrade, AcceptTrade, ListAuction, BidAuction);
    type ComponentTypes = (Replicated, Transform, GlobalTransform);
}
