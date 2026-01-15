//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + ML fact verification mercy eternal supreme immaculate
//! Chat filter (keyword + regex + ML truth scoring), adaptive learning, RON persistence philotic mercy

use bevy::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

const WHITELIST_FILE: &str = "mercy_shield_whitelist.ron";
const BLACKLIST_FILE: &str = "mercy_shield_blacklist.ron";
const FACTS_FILE: &str = "mercy_shield_facts.ron";

#[derive(Resource, Serialize, Deserialize)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct MLTruthFacts {
    pub facts: HashMap<String, (u32, u32)>,  // (true_count, false_count) mercy eternal
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

    let mut facts = HashMap::new();
    facts.insert("Earth is flat".to_string(), (0, 10));
    facts.insert("Sun rises in east".to_string(), (10, 0));
    // ... expanded mercy

    // Load persistent facts mercy eternal
    let mut loaded_facts = HashMap::new();
    if let Ok(contents) = fs::read_to_string(FACTS_FILE) {
        if let Ok(loaded) = ron::from_str::<HashMap<String, (u32, u32)>>(&contents) {
            loaded_facts = loaded;
        }
    }

    commands.insert_resource(ScamPatterns {
        keywords,
        regex_patterns,
        url_regex: Regex::new(r"https?://\S+").unwrap(),
        phone_regex: Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
    });

    commands.insert_resource(MLTruthFacts { facts: loaded_facts });

    let mut config = MercyShieldConfig {
        chat_sensitivity: 0.7,
        trade_sanity_check: true,
        auto_ban_threshold: 5,
        blacklist: HashSet::new(),
        whitelist_phrases: HashSet::new(),
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

    commands.insert_resource(config);
}

pub fn save_persistent_data_on_exit(
    config: Res<MercyShieldConfig>,
    truth_facts: Res<MLTruthFacts>,
) {
    if config.is_changed() || truth_facts.is_changed() {
        let pretty = ron::ser::PrettyConfig::new();

        let _ = fs::write(WHITELIST_FILE, ron::ser::to_string_pretty(&config.whitelist_phrases, pretty.clone()).unwrap_or_default());
        let _ = fs::write(BLACKLIST_FILE, ron::ser::to_string_pretty(&config.blacklist, pretty.clone()).unwrap_or_default());
        let _ = fs::write(FACTS_FILE, ron::ser::to_string_pretty(&truth_facts.facts, pretty).unwrap_or_default());
    }
}

pub fn ml_fact_verification_system(
    // Chat message events mercy — placeholder
    mut truth_facts: ResMut<MLTruthFacts>,
) {
    let message = "example message mercy";

    let mut truth_score = 0.5;  // Neutral start mercy

    for (fact, (true_count, false_count)) in &truth_facts.facts {
        if message.to_lowercase().contains(&fact.to_lowercase()) {
            let total = *true_count + *false_count;
            if total > 0 {
                truth_score = *true_count as f32 / total as f32;
            }
        }
    }

    // Use truth_score mercy eternal
}

pub fn ml_learning_from_report_system(
    // Report events mercy — true/false feedback
    mut truth_facts: ResMut<MLTruthFacts>,
    // Message + verdict mercy
) {
    let message = "reported message mercy";
    let is_true = true;  // From report mercy

    for word in message.to_lowercase().split_whitespace() {
        let entry = truth_facts.facts.entry(word.to_string()).or_insert((0, 0));
        if is_true {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_persistent_data_on_exit)
            .add_systems(Update, (
                chat_scam_filter_system,
                ml_fact_verification_system,
            ));
    }
}
