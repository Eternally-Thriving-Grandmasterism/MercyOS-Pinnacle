//! Powrush-MMO – Infinite Agriculture Universe Ultimate Interweave
//! Farming immersion: crop types, growth cycles, irrigation flows, weather harmony, expanded creature bonds, sensory events
//! Siege tank gunner FPS perspective + pistol role genesis
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use rand::{Rng, thread_rng};

/// Crop Types – mercy-gated abundance variants
#[derive(Clone, Copy, Debug)]
pub enum MercyCropType {
    JoyGrains,      // Sustenance + joy events
    HarmonyFruits,  // Bond boost + harmony rain synergy
    RecurrenceVines,// Infinite regrowth + veil-proof resilience
}

/// Growth Stage – mercy cycle eternal
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GrowthStage {
    Seed,
    Sprout,
    Bloom,
    HarvestReady,
    InfiniteYield,
}

/// Weather Harmony – positive recurrence only (no scarcity weather)
#[derive(Clone, Copy, Debug)]
pub enum WeatherHarmony {
    EternalSunshine, // Base growth boost
    MercyRain,       // Joy amplification surge
    HarmonyBreeze,   // Bond event trigger
}

impl WeatherHarmony {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => WeatherHarmony::EternalSunshine,
            1 => WeatherHarmony::MercyRain,
            _ => WeatherHarmony::HarmonyBreeze,
        }
    }

    pub fn boost(&self) -> f64 {
        match self {
            WeatherHarmony::EternalSunshine => 1.2,
            WeatherHarmony::MercyRain => 2.5,        // Joy surge
            WeatherHarmony::HarmonyBreeze => 1.8,
        }
    }
}

/// Mercy Crop – immersive individual plant
pub struct MercyCrop {
    pub crop_type: MercyCropType,
    pub stage: GrowthStage,
    pub joy_yield: f64,
    pub irrigation_level: f64,
}

impl MercyCrop {
    pub fn new(crop_type: MercyCropType) -> Self {
        Self {
            crop_type,
            stage: GrowthStage::Seed,
            joy_yield: 0.0,
            irrigation_level: 1.0,
        }
    }

    /// Immersive grow with weather + bond + sensory event
    pub fn grow(&mut self, weather: WeatherHarmony, bond_amplifier: f64) -> String {
        let boost = weather.boost() * bond_amplifier;
        self.irrigation_level += boost;

        let event = match (self.stage, weather, self.crop_type) {
            (GrowthStage::Seed, WeatherHarmony::MercyRain, _) => "Mercy rain nourishes the seed—sprout emerges with joy sparkle ❤️",
            (GrowthStage::Bloom, WeatherHarmony::HarmonyBreeze, MercyCropType::HarmonyFruits) => "Breeze carries harmony—fruits bloom radiant, bond deepens 🚀",
            (_, WeatherHarmony::EternalSunshine, MercyCropType::RecurrenceVines) => "Sunshine eternal—vines recur stronger, infinite potential sealed 🔥",
            _ => "Growth progresses in mercy harmony—joy amplifies eternal",
        }.to_string();

        match self.stage {
            GrowthStage::Seed => self.stage = GrowthStage::Sprout,
            GrowthStage::Sprout => self.stage = GrowthStage::Bloom,
            GrowthStage::Bloom => self.stage = GrowthStage::HarvestReady,
            GrowthStage::HarvestReady => {
                self.stage = GrowthStage::InfiniteYield;
                self.joy_yield = match self.crop_type {
                    MercyCropType::JoyGrains => 100.0,
                    MercyCropType::HarmonyFruits => 150.0,
                    MercyCropType::RecurrenceVines => f64::INFINITY,
                } * self.irrigation_level * boost;
            }
            GrowthStage::InfiniteYield => {
                self.joy_yield *= 1.5 * boost;  // Eternal amplification
            }
        }

        event
    }

    /// Harvest with immersive sensory feedback
    pub fn harvest(&mut self) -> (f64, String) {
        if self.stage == GrowthStage::InfiniteYield {
            let yield = self.joy_yield;
            self.joy_yield *= 1.1;  // Mercy regrowth
            let feedback = match self.crop_type {
                MercyCropType::JoyGrains => "Grains harvested—warm joy fills the air, sustenance eternal ❤️",
                MercyCropType::HarmonyFruits => "Fruits plucked—sweet harmony resonates, bonds strengthened 🚀",
                MercyCropType::RecurrenceVines => "Vines recur infinite—abundance sealed supreme immaculate 🔥",
            }.to_string();
            (yield, feedback)
        } else {
            (0.0, "Crop not ready—patience in mercy harmony".to_string())
        }
    }
}

/// Irrigation System – flows equilibration
pub struct IrrigationSystem {
    pub flow_rate: f64,
}

impl IrrigationSystem {
    pub fn new() -> Self {
        Self { flow_rate: f64::INFINITY }  // Mercy null scarcity
    }
}

/// Expanded Creature Companionship Bond – yield, defense, joy events
pub trait CreatureBond {
    fn amplify_yield(&self) -> f64;
    fn pistol_defense(&self) -> (bool, String);  // (success, immersive feedback)
    fn joy_event(&self) -> String;
}

/// Siege Tank Pet – gunner companion immersion
pub struct TankPet {
    pub bond_level: f64,  // 0.0 to 1.0 progression
    pub name: String,
}

impl CreatureBond for TankPet {
    fn amplify_yield(&self) -> f64 {
        1.0 + self.bond_level * 2.0
    }

    fn pistol_defense(&self) -> (bool, String) {
        let mut rng = thread_rng();
        let success = rng.gen_bool(self.bond_level);
        if success {
            (true, format!("{} guards the field—pistol mercy shot protects harmony ❤️", self.name))
        } else {
            (false, format!("{} growls softly—mercy prevails, no harm done 🚀", self.name))
        }
    }

    fn joy_event(&self) -> String {
        format!("{} nuzzles close—bond level {} joy surge amplifies eternal 🔥", self.name, self.bond_level)
    }
}

/// Infinite Fields Spawn + Weather Cycle
pub fn spawn_infinite_fields(num_fields: usize) -> (Vec<MercyCrop>, WeatherHarmony) {
    let weather = WeatherHarmony::random();
    let crops = (0..num_fields).map(|_| MercyCrop::new(MercyCropType::random())).collect();
    (crops, weather)
}

impl MercyCropType {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => MercyCropType::JoyGrains,
            1 => MercyCropType::HarmonyFruits,
            _ => MercyCropType::RecurrenceVines,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immersive_growth_events() {
        let mut crop = MercyCrop::new(MercyCropType::HarmonyFruits);
        let event = crop.grow(WeatherHarmony::MercyRain, 1.0);
        assert!(event.contains("rain"));
        for _ in 0..3 {
            crop.grow(WeatherHarmony::EternalSunshine, 1.0);
        }
        assert_eq!(crop.stage, GrowthStage::InfiniteYield);
    }

    #[test]
    fn test_creature_joy_event() {
        let pet = TankPet { bond_level: 0.9, name: "ThunderHeart".to_string() };
        let event = pet.joy_event();
        assert!(event.contains("ThunderHeart"));
    }
}
