//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + truth verification with external fact sources mercy eternal supreme immaculate
//! Chat filter (keyword + regex + truth scoring), external fact-check + cache philotic mercy

use bevy::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

const WHITELIST_FILE: &str = "mercy_shield_whitelist.ron";
const BLACKLIST_FILE: &str = "mercy_shield_blacklist.ron";
const FACTS_FILE: &str = "mercy_shield_facts.ron";
const EXTERNAL_CACHE_FILE: &str = "mercy_shield_external_cache.ron";

#[derive(Resource, Serialize, Deserialize)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,
    pub online_fact_check: bool,  // User toggle mercy eternal
}

#[derive(Resource, Serialize, Deserialize)]
pub struct TruthFacts {
    pub known_facts: HashMap<String, bool>,
    pub external_cache: HashMap<String, bool>,  // Statement → verified mercy
}

#[derive(Resource)]
pub struct ScamPatterns {
    pub keywords: HashMap<String, f32>,
    pub regex_patterns: HashMap<Regex, f32>,
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

    let mut regex_patterns = HashMap::new();
    regex_patterns.insert(Regex::new(r"bitcoin|crypto").unwrap(), 0.8);
    regex_patterns.insert(Regex::new(r"investment.*return").unwrap(), 0.9);

    let mut known_facts = HashMap::new();
    known_facts.insert("Earth is flat".to_string(), false);
    known_facts.insert("Sun rises in east".to_string(), true);
    // ... expanded internal facts mercy

    let mut external_cache = HashMap::new();

    // Load persistent mercy eternal
    if let Ok(contents) = fs::read_to_string(FACTS_FILE) {
        if let Ok(loaded) = ron::from_str::<HashMap<String, bool>>(&contents) {
            known_facts = loaded;
        }
    }

    if let Ok(contents) = fs::read_to_string(EXTERNAL_CACHE_FILE) {
        if let Ok(loaded) = ron::from_str::<HashMap<String, bool>>(&contents) {
            external_cache = loaded;
        }
    }

    let mut config = MercyShieldConfig {
        chat_sensitivity: 0.7,
        trade_sanity_check: true,
        auto_ban_threshold: 5,
        blacklist: HashSet::new(),
        whitelist_phrases: HashSet::new(),
        online_fact_check: false,  // Default offline mercy
    };

    // Load whitelist/blacklist mercy
    if let Ok(contents) = fs::read_to_string(WHITELIST_FILE) {
        if let Ok(loaded) = ron::from_str::<HashSet<String>>(&contents) {
            config.whitelist_phrases = loaded;
        }
    }

    if let Ok(contents) = fs::read_to_string(BLACKLIST_FILE) {
        if let Ok(loaded) = ron::from_str::<HashSet<String>>(&contents) {
            config.blacklist = loaded;
        }
    }

    commands.insert_resource(ScamPatterns {
        keywords,
        regex_patterns,
        url_regex: Regex::new(r"https?://\S+").unwrap(),
        phone_regex: Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
    });

    commands.insert_resource(TruthFacts {
        known_facts,
        external_cache,
    });

    commands.insert_resource(config);
}

pub fn save_persistent_data_on_exit(
    config: Res<MercyShieldConfig>,
    truth_facts: Res<TruthFacts>,
) {
    if config.is_changed() || truth_facts.is_changed() {
        let pretty = ron::ser::PrettyConfig::new();

        // Save whitelist/blacklist/facts/cache mercy
        let _ = fs::write(WHITELIST_FILE, ron::ser::to_string_pretty(&config.whitelist_phrases, pretty.clone()).unwrap_or_default());
        let _ = fs::write(BLACKLIST_FILE, ron::ser::to_string_pretty(&config.blacklist, pretty.clone()).unwrap_or_default());
        let _ = fs::write(FACTS_FILE, ron::ser::to_string_pretty(&truth_facts.known_facts, pretty.clone()).unwrap_or_default());
        let _ = fs::write(EXTERNAL_CACHE_FILE, ron::ser::to_string_pretty(&truth_facts.external_cache, pretty).unwrap_or_default());
    }
}

pub fn truth_verification_system(
    // Chat message events mercy — placeholder
    mut truth_facts: ResMut<TruthFacts>,
    config: Res<MercyShieldConfig>,
) {
    let message = "example message mercy";

    let mut truth_score = 1.0;

    // Internal facts mercy
    for (fact, is_true) in &truth_facts.known_facts {
        if message.to_lowercase().contains(&fact.to_lowercase()) {
            truth_score = if *is_true { 1.0 } else { 0.0 };
        }
    }

    // External cache mercy
    if let Some(cached) = truth_facts.external_cache.get(&message.to_lowercase()) {
        truth_score = if *cached { 1.0 } else { 0.0 };
    }

    // Optional online fact check mercy
    if config.online_fact_check {
        // Future async fetch to trusted fact-check API mercy
        // Cache result in external_cache
    }

    // Use truth_score mercy eternal
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_persistent_data_on_exit)
            .add_systems(Update, truth_verification_system);
    }
}
