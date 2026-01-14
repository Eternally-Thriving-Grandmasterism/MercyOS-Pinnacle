//! powrush_mmo/auction.rs — Complete global auction house mercy
//! General auctions + dedicated creature bidding with ownership transfer

use bevy::prelude::*;
use lightyear::prelude::ClientId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AuctionHouse {
    pub general_auctions: HashMap<u64, GeneralAuction>,
    pub creature_auctions: HashMap<u64, CreatureAuction>,
    pub next_id: u64,
}

#[derive(Clone, Debug)]
pub struct CreatureAuction {
    pub seller: ClientId,
    pub creature_entity: Entity,
    pub creature_dna: CreatureDNA,
    pub creature_type: CreatureType,
    pub current_bid: u32,
    pub current_bidder: Option<ClientId>,
    pub end_time: f64,
}

// auction_tick_system checks creature_auctions, on end:
fn creature_auction_end(
    auction_house: &mut ResMut<AuctionHouse>,
    creature_query: &mut Query<&mut Creature>,
    player_query: &mut Query<&mut Player>,
    winner: ClientId,
    auction_id: u64,
) {
    if let Some(auction) = auction_house.creature_auctions.remove(&auction_id) {
        if let Ok(mut creature) = creature_query.get_mut(auction.creature_entity) {
            creature.owner = Some(/* winner entity mercy */);
            creature.state = CreatureState::Follow;
            creature.tamed = true;
            // Add to winner's tamed_creatures
            // Golden paw + heart particles broadcast
        }
    }
}
