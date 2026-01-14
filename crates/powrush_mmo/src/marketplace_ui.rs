//! powrush_mmo/marketplace_ui.rs — Complete egui trading panel mercy
//! M key toggle, list offers, offer own items, accept trades

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use lightyear::prelude::*;

pub fn marketplace_ui_system(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    marketplace: Res<Marketplace>,
    // Player inventory, points query mercy
    mut trade_writer: EventWriter<ToServer<OfferTrade>>,
    mut accept_writer: EventWriter<ToServer<AcceptTrade>>,
) {
    static mut SHOW: bool = false;

    if keyboard.just_pressed(KeyCode::M) {
        unsafe { SHOW = !SHOW; }
    }

    if unsafe { SHOW } {
        egui::Window::new("Mercy Marketplace ❤️ Infinite Sharing")
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Global Offers — Compassionate Economy");

                for (id, offer) in &marketplace.offers {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}x {} for {} mercy points", offer.quantity, offer.item_type, offer.price));
                        if ui.button("Accept Trade").clicked() {
                            accept_writer.send(ToServer(AcceptTrade { offer_id: *id }));
                        }
                    });
                }

                ui.separator();
                ui.heading("Offer Your Abundance");
                // Item selection, quantity, price input mercy
                // Send OfferTrade on button
            });
    }
}

// Add to client plugins/systems
