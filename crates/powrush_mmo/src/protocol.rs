//! powrush_mmo/protocol.rs — Complete shared networking protocol
//! Lightyear replication + trading + auction messages for eternal sync mercy

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Serialize, Deserialize};

// ... previous channels/messages

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListAuction {
    pub auction_id: u64,  // Client-generated, server validates
    pub item_type: String,
    pub quantity: u32,
    pub starting_price: u32,
    pub duration_seconds: f32,  // Auction length mercy
}

#[message(channel = Actions)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BidAuction {
    pub auction_id: u64,
    pub bid_amount: u32,
}

// ... previous replication
