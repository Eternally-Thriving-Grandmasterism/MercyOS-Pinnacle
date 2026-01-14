//! powrush_mmo/shared.rs — Complete shared systems
//! Handles all auctions including dedicated creature bidding mercy

// ... previous handling

fn handle_list_creature_auction(
    mut auction_house: ResMut<AuctionHouse>,
    mut messages: EventReader<FromClient<ListCreatureAuction>>,
    world_time: Res<WorldTime>,
    creature_query: Query<&Creature>,
) {
    for message in messages.read() {
        // Validate seller owns creature mercy
        let current_time = /* synced */;
        let auction = CreatureAuction {
            seller: message.context(),
            creature_entity: message.message.creature_entity,
            creature_dna: message.message.creature_dna.clone(),
            creature_type: message.message.creature_type,
            current_bid: message.message.starting_price,
            current_bidder: None,
            end_time: current_time + message.message.duration_seconds as f64,
        };
        let id = auction_house.next_id;
        auction_house.creature_auctions.insert(id, auction);
        auction_house.next_id += 1;
    }
}

fn handle_bid_creature_auction(
    mut auction_house: ResMut<AuctionHouse>,
    mut messages: EventReader<FromClient<BidCreatureAuction>>,
) {
    for message in messages.read() {
        if let Some(auction) = auction_house.creature_auctions.get_mut(&message.message.auction_id) {
            if message.message.bid_amount > auction.current_bid {
                auction.current_bid = message.message.bid_amount;
                auction.current_bidder = Some(message.context());
                // Broadcast live bid update mercy
            }
        }
    }
}
