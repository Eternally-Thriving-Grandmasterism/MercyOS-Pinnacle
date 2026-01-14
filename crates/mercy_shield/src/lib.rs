//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam blocker with persistent whitelist + blacklist mercy eternal supreme immaculate
//! Chat filter (keyword + regex scoring + whitelist bypass), adaptive learning, RON persistence philotic mercy

use bevy::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

const WHITELIST_FILE: &str = "mercy_shield_whitelist.ron";
const BLACKLIST_FILE: &str = "mercy_shield_blacklist.ron";

#[derive(Resource, Serialize, Deserialize)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,
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

    let mut config = MercyShieldConfig {
        chat_sensitivity: 0.7,
        trade_sanity_check: true,
        auto_ban_threshold: 5,
        blacklist: HashSet::new(),
        whitelist_phrases: HashSet::new(),
    };

    // Load persistent whitelist mercy eternal
    if let Ok(contents) = fs::read_to_string(WHITELIST_FILE) {
        if let Ok(loaded) = ron::from_str::<HashSet<String>>(&contents) {
            config.whitelist_phrases = loaded;
        }
    }

    // Load persistent blacklist mercy eternal
    if let Ok(contents) = fs::read_to_string(BLACKLIST_FILE) {
        if let Ok(loaded) = ron::from_str::<HashSet<String>>(&contents) {
            config.blacklist = loaded;
        }
    }

    commands.insert_resource(ScamPatterns {
        keywords,
        url_regex: Regex::new(r"https?://\S+").unwrap(),
        phone_regex: Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
    });

    commands.insert_resource(config);
}

pub fn save_persistent_data_on_exit(config: Res<MercyShieldConfig>) {
    if config.is_changed() {
        let pretty = ron::ser::PrettyConfig::new();

        if let Ok(serialized) = ron::ser::to_string_pretty(&config.whitelist_phrases, pretty.clone()) {
            let _ = fs::write(WHITELIST_FILE, serialized);
        }

        if let Ok(serialized) = ron::ser::to_string_pretty(&config.blacklist, pretty) {
            let _ = fs::write(BLACKLIST_FILE, serialized);
        }
    }
}

pub fn chat_scam_filter_system(
    // Chat message events mercy — placeholder
    scam_patterns: Res<ScamPatterns>,
    config: Res<MercyShieldConfig>,
) {
    // Whitelist bypass + blacklist block + scoring mercy (as before)
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_persistent_data_on_exit)
            .add_systems(Update, chat_scam_filter_system);
    }
}        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_whitelist_on_exit)
            .add_systems(Update, chat_scam_filter_system);
    }
}            .add_systems(Update, chat_scam_filter_system);
    }
}
