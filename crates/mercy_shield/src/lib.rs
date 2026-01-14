//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam blocker with ML-like pattern scoring mercy eternal supreme immaculate
//! Chat filter (keyword + regex scoring), trade validation, player report quorum philotic mercy

use bevy::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Resource)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,  // 0.0 permissive - 1.0 strict mercy
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
}

#[derive(Resource)]
pub struct ScamPatterns {
    pub keywords: HashMap<String, f32>,  // Word → weight mercy
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
    });
}

pub fn chat_scam_filter_system(
    // Chat message events mercy — placeholder
    scam_patterns: Res<ScamPatterns>,
    config: Res<MercyShieldConfig>,
) {
    // Example scoring mercy
    let message = "Urgent! Verify your bank password here: http://fake.com/free-money";

    let mut score = 0.0;

    for (word, weight) in &scam_patterns.keywords {
        if message.to_lowercase().contains(word) {
            score += weight;
        }
    }

    if scam_patterns.url_regex.is_match(message) {
        score += 0.8;
    }

    if scam_patterns.phone_regex.is_match(message) {
        score += 0.6;
    }

    let threshold = config.chat_sensitivity * 2.0;  // Scale mercy

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
