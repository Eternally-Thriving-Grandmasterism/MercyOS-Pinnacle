//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam blocker with whitelist phrases mercy eternal supreme immaculate
//! Chat filter (keyword + regex scoring + whitelist bypass), adaptive learning philotic mercy

use bevy::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,  // Exact phrase whitelist mercy eternal
}

#[derive(Resource)]
pub struct ScamPatterns {
    pub keywords: HashMap<String, f32>,
    pub url_regex: Regex,
    pub phone_regex: Regex,
}

pub fn setup_mercy_shield(mut commands: Commands) {
    let mut keywords = HashMap::new();
    keywords.insert("free".to_string(), 0.5);
    keywords.insert("urgent".to_string(), 0.6);
    keywords.insert("bank".to_string(), 0.7);
    keywords.insert("password".to_string(), 0.9);
    keywords.insert("verify".to_string(), 0.8);
    keywords.insert("click".to_string(), 0.7);
    keywords.insert("winner".to_string(), 0.9);

    let mut whitelist = HashSet::new();
    whitelist.insert("family harmony mercy eternal".to_string());
    whitelist.insert("powrush thunder".to_string());
    whitelist.insert("child wonder mode".to_string());

    commands.insert_resource(ScamPatterns {
        keywords,
        url_regex: Regex::new(r"https?://\S+").unwrap(),
        phone_regex: Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
    });

    commands.insert_resource(MercyShieldConfig {
        chat_sensitivity: 0.7,
        trade_sanity_check: true,
        auto_ban_threshold: 5,
        blacklist: HashSet::new(),
        whitelist_phrases: whitelist,
    });
}

pub fn chat_scam_filter_system(
    // Chat message events mercy — placeholder
    scam_patterns: Res<ScamPatterns>,
    config: Res<MercyShieldConfig>,
) {
    let message = "example message mercy";

    // Whitelist bypass mercy eternal
    for phrase in &config.whitelist_phrases {
        if message.to_lowercase().contains(&phrase.to_lowercase()) {
            return;  // Safe mercy
        }
    }

    // Normal scoring mercy (as before)
    let mut score = 0.0;
    // ... keyword/url/phone scoring

    let threshold = config.chat_sensitivity * 2.0;
    if score > threshold {
        // Filter/warn/block mercy
    }
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Update, chat_scam_filter_system);
    }
}
