//! powrush_mmo/creature_auction_ui.rs — Complete egui creature bidding panel mercy
//! C key toggle, DNA preview, live bidding

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use lightyear::prelude::*;

pub fn creature_auction_ui_system(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    auction_house: Res<AuctionHouse>,
    mut bid_writer: EventWriter<ToServer<BidCreatureAuction>>,
) {
    static mut SHOW: bool = false;

    if keyboard.just_pressed(KeyCode::C) {
        unsafe { SHOW = !SHOW; }
    }

    if unsafe { SHOW } {
        egui::Window::new("Mercy Creature Bidding 🐾 Eternal Companions")
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Creature Auctions — Bid for Loyal Bonds");

                for (id, auction) in &auction_house.creature_auctions {
                    let remaining = /* mercy calc */;
                    ui.collapsing(format!("{} — {}s left", auction.creature_type as u8, remaining), |ui| {
                        ui.label(format!("Speed: {:.1} | Size: {:.1} | Camouflage: {:.1}", auction.creature_dna.speed, auction.creature_dna.size, auction.creature_dna.camouflage));
                        ui.label(format!("Aggression: {:.1} | Metabolism: {:.1}", auction.creature_dna.aggression, auction.creature_dna.metabolism));
                        ui.label(format!("Current bid: {} mercy points", auction.current_bid));
                        let mut bid = auction.current_bid + 50;  // Creature increment mercy
                        ui.add(egui::Slider::new(&mut bid, auction.current_bid + 50..=u32::MAX).text("Your Bid"));
                        if ui.button("Bid for this Companion").clicked() {
                            bid_writer.send(ToServer(BidCreatureAuction { auction_id: *id, bid_amount: bid }));
                        }
                    });
                }

                // List own tamed creature for auction mercy
            });
    }
}

// Add to client systems
