//! powrush_mmo/auction_ui.rs — Complete egui auction bidding panel mercy
//! A key toggle, list active auctions, bid live, list own

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use lightyear::prelude::*;

pub fn auction_ui_system(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    auction_house: Res<AuctionHouse>,
    mut bid_writer: EventWriter<ToServer<BidAuction>>,
    mut list_writer: EventWriter<ToServer<ListAuction>>,
) {
    static mut SHOW: bool = false;

    if keyboard.just_pressed(KeyCode::A) {
        unsafe { SHOW = !SHOW; }
    }

    if unsafe { SHOW } {
        egui::Window::new("Mercy Auction House ⚒️ Compassionate Bidding")
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Active Auctions — Highest Bid Wins Eternal");

                for (id, auction) in &auction_house.auctions {
                    let remaining = (auction.end_time - current_server_time()) as i32;  // Mercy calc
                    ui.horizontal(|ui| {
                        ui.label(format!("{}x {} — Current: {} points ({}s left)", auction.quantity, auction.item_type, auction.current_bid, remaining));
                        let mut bid = auction.current_bid + 10;  // Min increment mercy
                        ui.add(egui::Slider::new(&mut bid, auction.current_bid + 10..=u32::MAX).text("Bid"));
                        if ui.button("Place Bid").clicked() {
                            bid_writer.send(ToServer(BidAuction { auction_id: *id, bid_amount: bid }));
                        }
                    });
                }

                ui.separator();
                ui.heading("List New Auction");
                // Item, quantity, start price, duration input mercy
                // Send ListAuction on button
            });
    }
}
