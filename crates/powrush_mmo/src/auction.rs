//! powrush_mmo/auction.rs — Global auction bidding mercy
//! Time-limited auctions, server-authoritative bidding/winner
//! Compassionate competition eternal

use bevy::prelude::*;
use lightyear::prelude::ClientId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AuctionHouse {
    pub auctions: HashMap<u64, ActiveAuction>,
    pub next_id: u64,
}

#[derive(Clone, Debug)]
pub struct ActiveAuction {
    pub seller: ClientId,
    pub item_type: String,
    pub quantity: u32,
    pub current_bid: u32,
    pub current_bidder: Option<ClientId>,
    pub end_time: f64,  // WorldTime seconds mercy
}

pub fn auction_tick_system(
    mut auction_house: ResMut<AuctionHouse>,
    time: Res<Time>,
    world_time: Res<WorldTime>,  // Synced server time
    // Transfer on end mercy
) {
    let current_time = world_time.day as f64 * 86400.0 + world_time.time_of_day as f64;

    let mut ended = vec![];
    for (id, auction) in &mut auction_house.auctions {
        if current_time >= auction.end_time {
            ended.push(*id);
            // Handle winner transfer, golden hammer particles broadcast
        }
    }
    for id in ended {
        auction_house.auctions.remove(&id);
    }
}
