//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + dynamic threshold adaptation mercy eternal supreme immaculate
//! Chat filter (keyword + regex + Bayesian truth scoring), adaptive learning + sensitivity auto-tune philotic mercy

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
pub struct BayesianTruthFacts {
    pub facts: HashMap<String, (u32, u32)>,
}

#[derive(Resource)]
pub struct ScamPatterns {
    pub keywords: HashMap<String, f32>,
    pub regex_patterns: HashMap<Regex, f32>,
    pub url_regex: Regex,
    pub phone_regex: Regex,
}

#[derive(Resource)]
pub struct AdaptationMetrics {
    pub recent_true_positives: u32,
    pub recent_false_positives: u32,
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
    known_facts.insert("Sun rises in east".to_string(), (11, 1));

    let mut config = MercyShieldConfig {
        chat_sensitivity: 0.7,
        trade_sanity_check: true,
        auto_ban_threshold: 5,
        blacklist: HashSet::new(),
        whitelist_phrases: HashSet::new(),
    };

    // Load persistent mercy eternal
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

    commands.insert_resource(BayesianTruthFacts { facts: known_facts });

    commands.insert_resource(AdaptationMetrics {
        recent_true_positives: 0,
        recent_false_positives: 0,
    });

    commands.insert_resource(config);
}

pub fn dynamic_threshold_adaptation_system(
    mut config: ResMut<MercyShieldConfig>,
    mut metrics: ResMut<AdaptationMetrics>,
    time: Res<Time>,
) {
    // Simple EMA mercy — adjust sensitivity
    let total_reports = metrics.recent_true_positives + metrics.recent_false_positives;
    if total_reports > 0 {
        let fp_rate = metrics.recent_false_positives as f32 / total_reports as f32;

        // Too many false positives → loosen
        if fp_rate > 0.3 {
            config.chat_sensitivity -= 0.05 * time.delta_seconds();
            config.chat_sensitivity = config.chat_sensitivity.max(0.3);
        }
        // Too few detections → tighten
        if fp_rate < 0.1 {
            config.chat_sensitivity += 0.03 * time.delta_seconds();
            config.chat_sensitivity = config.chat_sensitivity.min(0.9);
        }
    }

    // Reset metrics periodically mercy
    // Future: rolling window
}

pub fn report_feedback_system(
    // Report events mercy — true/false
    mut metrics: ResMut<AdaptationMetrics>,
) {
    let is_scam = true;  // From report mercy

    if is_scam {
        metrics.recent_true_positives += 1;
    } else {
        metrics.recent_false_positives += 1;
    }
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Update, (
                chat_scam_filter_system,
                dynamic_threshold_adaptation_system,
                report_feedback_system,
            ));
    }
}        raw_implications,
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
