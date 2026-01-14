//! powrush_mmo/marketplace.rs — Global trading marketplace mercy
//! Server-authoritative offers, infinite mercy points economy
//! No scarcity — compassionate sharing eternal

use bevy::prelude::*;
use lightyear::prelude::ClientId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Marketplace {
    pub offers: HashMap<u64, TradeOffer>,
    pub next_id: u64,
}

#[derive(Clone, Debug)]
pub struct TradeOffer {
    pub seller: ClientId,
    pub item_type: String,
    pub quantity: u32,
    pub price: u32,  // Mercy points — infinite sharing
}

#[derive(Resource, Default)]
pub struct PlayerMercyPoints {
    pub points: HashMap<ClientId, u64>,  // Infinite start mercy
}
