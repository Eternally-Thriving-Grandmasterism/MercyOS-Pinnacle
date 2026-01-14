//! powrush_mmo/protocol.rs — Complete shared networking protocol
//! Lightyear replication + trading + general auctions + dedicated creature bidding mercy

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Serialize, Deserialize};

// ... previous channels/messages (PlantCrop, OfferTrade, AcceptTrade, ListAuction, BidAuction)

// Dedicated creature bidding messages
#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListCreatureAuction {
    pub auction_id: u64,
    pub creature_entity: Entity,  // Server reference mercy
    pub creature_dna: CreatureDNA,
    pub creature_type: CreatureType,
    pub starting_price: u32,
    pub duration_seconds: f32,
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BidCreatureAuction {
    pub auction_id: u64,
    pub bid_amount: u32,
}

// BidAuction can be shared, but dedicated for clarity mercy
