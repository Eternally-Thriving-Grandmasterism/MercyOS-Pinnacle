//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam blocker with adaptive ML training mercy eternal supreme immaculate
//! Chat filter (keyword + regex scoring), false positive feedback learning, trade validation philotic mercy

use bevy::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
}

#[derive(Resource)]
pub struct ScamPatterns {
    pub keywords: HashMap<String, f32>,  // Word → weight (adaptive mercy)
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
    // Chat message events mercy — placeholder scoring
    scam_patterns: Res<ScamPatterns>,
    config: Res<MercyShieldConfig>,
) {
    // Scoring logic mercy (as before)
}

pub fn false_positive_learning_system(
    // False positive report events mercy — player marks flagged message "not scam"
    mut scam_patterns: ResMut<ScamPatterns>,
    // Reported message mercy
) {
    let message = "example false positive message mercy";

    for word in message.to_lowercase().split_whitespace() {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !cleaned.is_empty() {
            if let Some(weight) = scam_patterns.keywords.get_mut(&cleaned.to_string()) {
                *weight = (*weight - 0.15).max(0.1);  // Decrease but floor mercy eternal
            }
        }
    }
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Update, (
                chat_scam_filter_system,
                player_report_learning_system,
                false_positive_learning_system,
            ));
    }
}
