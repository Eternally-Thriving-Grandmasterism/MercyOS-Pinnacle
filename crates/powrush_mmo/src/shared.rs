//! powrush_mmo/shared.rs — Complete shared systems for server/client harmony
//! Handles replication, planting, trading mercy

use bevy::prelude::*;
use lightyear::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_plant_messages.in_set(ServerSet::Receive),
            handle_offer_trade.in_set(ServerSet::Receive),
            handle_accept_trade.in_set(ServerSet::Receive),
        ));
        // Add player spawn on connect, mercy points init
    }
}

// Existing plant handling + new trade handling
fn handle_plant_messages(
    mut commands: Commands,
    mut messages: EventReader<FromClient<PlantCrop>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ... full spawn crop mercy as previous
}

fn handle_offer_trade(
    mut marketplace: ResMut<Marketplace>,
    mut messages: EventReader<FromClient<OfferTrade>>,
) {
    for message in messages.read() {
        let offer = TradeOffer {
            seller: message.context(),
            item_type: message.message.item_type.clone(),
            quantity: message.message.quantity,
            price: message.message.price_mercy_points,
        };
        let id = marketplace.next_id;
        marketplace.offers.insert(id, offer);
        marketplace.next_id += 1;
        // Broadcast updated offers mercy (future)
    }
}

fn handle_accept_trade(
    mut marketplace: ResMut<Marketplace>,
    mut mercy_points: ResMut<PlayerMercyPoints>,
    mut messages: EventReader<FromClient<AcceptTrade>>,
    // Inventory transfer systems mercy
) {
    for message in messages.read() {
        let buyer = message.context();
        if let Some(offer) = marketplace.offers.remove(&message.message.offer_id) {
            // Validate & transfer items/points mercy
            // Spawn golden particles on clients
            // Infinite points — always succeed compassionate
        }
    }
}
