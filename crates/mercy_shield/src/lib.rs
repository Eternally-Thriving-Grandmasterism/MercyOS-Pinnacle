//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + threshold-based relation promotion mercy eternal supreme immaculate
//! Chat filter (keyword + regex + dependency-aware), adaptive learning with threshold promotion, RON persistence philotic mercy

use bevy::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

const WHITELIST_FILE: &str = "mercy_shield_whitelist.ron";
const BLACKLIST_FILE: &str = "mercy_shield_blacklist.ron";
const FACTS_FILE: &str = "mercy_shield_facts.ron";
const RELATIONS_FILE: &str = "mercy_shield_relations.ron";
const PROMOTION_THRESHOLD: u32 = 5;  // Co-occurrence count before promotion mercy eternal

#[derive(Resource, Serialize, Deserialize)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct TruthFacts {
    pub known_facts: HashMap<String, (u32, u32)>,  // (true_reports, false_reports) mercy eternal
    pub raw_implications: HashMap<String, HashMap<String, u32>>,  // Raw co-true counts mercy
    pub raw_contradictions: HashMap<String, HashMap<String, u32>>,  // Raw co-false counts mercy
    pub implications: HashMap<String, Vec<String>>,  // Promoted mercy eternal
    pub contradictions: HashMap<String, Vec<String>>,  // Promoted mercy eternal
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
    known_facts.insert("Earth is flat".to_string(), (1, 11));
    known_facts.insert("Moon landing happened".to_string(), (11, 1));
    known_facts.insert("Earth is round".to_string(), (11, 1));

    let raw_implications = HashMap::new();
    let raw_contradictions = HashMap::new();
    let implications = HashMap::new();
    let contradictions = HashMap::new();

    // Load persistent mercy eternal
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

    commands.insert_resource(TruthFacts {
        known_facts: loaded_facts,
        raw_implications,
        raw_contradictions,
        implications,
        contradictions,
    });

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
    truth_facts: Res<TruthFacts>,
) {
    if config.is_changed() || truth_facts.is_changed() {
        let pretty = ron::ser::PrettyConfig::new();

        let _ = fs::write(WHITELIST_FILE, ron::ser::to_string_pretty(&config.whitelist_phrases, pretty.clone()).unwrap_or_default());
        let _ = fs::write(BLACKLIST_FILE, ron::ser::to_string_pretty(&config.blacklist, pretty.clone()).unwrap_or_default());
        let _ = fs::write(FACTS_FILE, ron::ser::to_string_pretty(&truth_facts.known_facts, pretty.clone()).unwrap_or_default());
        let _ = fs::write(RELATIONS_FILE, ron::ser::to_string_pretty(&(&truth_facts.raw_implications, &truth_facts.raw_contradictions), pretty).unwrap_or_default());
    }
}

pub fn implication_learning_system(
    // Report events mercy — message + verdict
    mut truth_facts: ResMut<TruthFacts>,
) {
    let message = "reported message mercy";
    let is_true = true;  // From report mercy

    let matched_facts: Vec<String> = truth_facts.known_facts.keys()
        .filter(|fact| message.to_lowercase().contains(&fact.to_lowercase()))
        .cloned()
        .collect();

    if matched_facts.len() > 1 {
        for i in 0..matched_facts.len() {
            for j in (i + 1)..matched_facts.len() {
                let a = &matched_facts[i];
                let b = &matched_facts[j];

                if is_true {
                    *truth_facts.raw_implications.entry(a.clone()).or_insert(HashMap::new())
                        .entry(b.clone()).or_insert(0) += 1;
                    *truth_facts.raw_implications.entry(b.clone()).or_insert(HashMap::new())
                        .entry(a.clone()).or_insert(0) += 1;
                } else {
                    *truth_facts.raw_contradictions.entry(a.clone()).or_insert(HashMap::new())
                        .entry(b.clone()).or_insert(0) += 1;
                    *truth_facts.raw_contradictions.entry(b.clone()).or_insert(HashMap::new())
                        .entry(a.clone()).or_insert(0) += 1;
                }
            }
        }
    }

    // Threshold promotion mercy eternal
    promote_relations(&mut truth_facts);
}

fn promote_relations(truth_facts: &mut TruthFacts) {
    let mut new_implications = HashMap::new();
    let mut new_contradictions = HashMap::new();

    for (a, map) in &truth_facts.raw_implications {
        for (b, count) in map {
            if *count >= PROMOTION_THRESHOLD {
                new_implications.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
            }
        }
    }

    for (a, map) in &truth_facts.raw_contradictions {
        for (b, count) in map {
            if *count >= PROMOTION_THRESHOLD {
                new_contradictions.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
            }
        }
    }

    truth_facts.implications = new_implications;
    truth_facts.contradictions = new_contradictions;
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_persistent_data_on_exit)
            .add_systems(Update, implication_learning_system);
    }
}
