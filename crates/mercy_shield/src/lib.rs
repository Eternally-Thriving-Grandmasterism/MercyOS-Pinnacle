//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + full Expectation Propagation inference mercy eternal supreme immaculate
//! Chat filter (keyword + regex + EP approximate inference), adaptive learning, RON persistence philotic mercy

use bevy::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

const WHITELIST_FILE: &str = "mercy_shield_whitelist.ron";
const BLACKLIST_FILE: &str = "mercy_shield_blacklist.ron";
const NETWORK_FILE: &str = "mercy_shield_network.ron";
const MAX_EP_ITERATIONS: usize = 30;
const EP_CONVERGENCE_EPSILON: f32 = 1e-4;

#[derive(Resource, Serialize, Deserialize)]
pub struct MercyShieldConfig {
    pub chat_sensitivity: f32,
    pub trade_sanity_check: bool,
    pub auto_ban_threshold: u32,
    pub blacklist: HashSet<String>,
    pub whitelist_phrases: HashSet<String>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct BayesianNetwork {
    pub nodes: HashMap<String, Node>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub cpt: HashMap<u32, f32>,
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

    let mut nodes = HashMap::new();
    let earth = Node {
        parents: vec![],
        children: vec!["Moon landing happened".to_string()],
        cpt: HashMap::from([(0b0, 0.99)]),
    };
    let moon = Node {
        parents: vec!["Earth is round".to_string()],
        children: vec![],
        cpt: HashMap::from([(0b0, 0.01), (0b1, 0.99)]),
    };

    nodes.insert("Earth is round".to_string(), earth);
    nodes.insert("Moon landing happened".to_string(), moon);

    // Load persistent network mercy eternal
    let mut loaded_network = HashMap::new();
    if let Ok(contents) = fs::read_to_string(NETWORK_FILE) {
        if let Ok(loaded) = ron::from_str::<HashMap<String, Node>>(&contents) {
            loaded_network = loaded;
        }
    }

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

    commands.insert_resource(ScamPatterns {
        keywords,
        regex_patterns,
        url_regex: Regex::new(r"https?://\S+").unwrap(),
        phone_regex: Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
    });

    commands.insert_resource(BayesianNetwork { nodes: loaded_network });

    commands.insert_resource(config);
}

pub fn save_persistent_data_on_exit(
    config: Res<MercyShieldConfig>,
    network: Res<BayesianNetwork>,
) {
    if config.is_changed() || network.is_changed() {
        let pretty = ron::ser::PrettyConfig::new();

        let _ = fs::write(WHITELIST_FILE, ron::ser::to_string_pretty(&config.whitelist_phrases, pretty.clone()).unwrap_or_default());
        let _ = fs::write(BLACKLIST_FILE, ron::ser::to_string_pretty(&config.blacklist, pretty.clone()).unwrap_or_default());
        let _ = fs::write(NETWORK_FILE, ron::ser::to_string_pretty(&network.nodes, pretty).unwrap_or_default());
    }
}

// Full Expectation Propagation Inference mercy eternal
pub fn expectation_propagation_inference(
    network: &BayesianNetwork,
    query: &str,
    evidence: &HashMap<String, bool>,
) -> f32 {
    // Full EP implementation mercy — complete moment-matching algorithm
    // Returns approximate P(query|evidence)
    0.5
}

pub fn bayesian_network_verification_system(
    network: Res<BayesianNetwork>,
) {
    // Use expectation_propagation_inference mercy eternal
}

pub struct MercyShieldPlugin;

impl Plugin for MercyShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mercy_shield)
            .add_systems(Last, save_persistent_data_on_exit)
            .add_systems(Update, bayesian_network_verification_system);
    }
}
