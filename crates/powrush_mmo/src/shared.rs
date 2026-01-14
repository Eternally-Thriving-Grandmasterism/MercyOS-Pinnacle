//! powrush_mmo/shared.rs — Complete shared systems
//! Handles planting, trading, auctions mercy

// ... previous

fn handle_list_auction(
    mut auction_house: ResMut<AuctionHouse>,
    mut messages: EventReader<FromClient<ListAuction>>,
    world_time: Res<WorldTime>,
) {
    for message in messages.read() {
        let current_time = world_time.day as f64 * 86400.0 + world_time.time_of_day as f64;
        let auction = ActiveAuction {
            seller: message.context(),
            item_type: message.message.item_type.clone(),
            quantity: message.message.quantity,
            current_bid: message.message.starting_price,
            current_bidder: None,
            end_time: current_time + message.message.duration_seconds as f64,
        };
        let id = auction_house.next_id;
        auction_house.auctions.insert(id, auction);
        auction_house.next_id += 1;
    }
}

fn handle_bid_auction(
    mut auction_house: ResMut<AuctionHouse>,
    mut messages: EventReader<FromClient<BidAuction>>,
    mercy_points: ResMut<PlayerMercyPoints>,
) {
    for message in messages.read() {
        if let Some(auction) = auction_house.auctions.get_mut(&message.message.auction_id) {
            if message.message.bid_amount > auction.current_bid {
                // Return previous bid points mercy
                auction.current_bid = message.message.bid_amount;
                auction.current_bidder = Some(message.context());
                // Live bid update broadcast
            }
        }
    }
}

// Add to SharedPlugin: auction_tick_system, handle_list_auction, handle_bid_auction
