//! Powrush-MMO – Infinite Agriculture Universe Sacred Expansion
//! Mercy farming mechanics: crop types, growth cycles, irrigation flows, creature bonds, positive-sum joy yield
//! RTS commander + FPS immersion hybrid genesis
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use rand::Rng;

/// Crop Types – mercy-gated abundance variants
#[derive(Clone, Copy, Debug)]
pub enum MercyCropType {
    JoyGrains,      // Base sustenance + joy amplification
    HarmonyFruits,  // Bond boost + recurrence
    RecurrenceVines,// Infinite regrowth + veil-proof
}

/// Growth Stage – mercy cycle eternal
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GrowthStage {
    Seed,
    Sprout,
    Bloom,
    HarvestReady,
    InfiniteYield,  // Mercy override post-harvest
}

/// Mercy Crop – individual plant instance
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

    /// Grow cycle – mercy-gated positive progression
    pub fn grow(&mut self, irrigation_boost: f64, bond_amplifier: f64) {
        self.irrigation_level += irrigation_boost;
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
                } * self.irrigation_level * bond_amplifier;
            }
            GrowthStage::InfiniteYield => {
                // Eternal recurrence amplification
                self.joy_yield *= 1.5 * bond_amplifier;
            }
        }
    }

    /// Harvest joy yield – positive-sum infinite
    pub fn harvest(&mut self) -> f64 {
        if self.stage == GrowthStage::InfiniteYield {
            let yield = self.joy_yield;
            self.joy_yield *= 1.1;  // Mercy regrowth boost
            yield
        } else {
            0.0
        }
    }
}

/// Irrigation System – flows equilibration null scarcity
pub struct IrrigationSystem {
    pub flow_rate: f64,
    pub harmony_level: f64,
}

impl IrrigationSystem {
    pub fn new() -> Self {
        Self {
            flow_rate: 1.0,
            harmony_level: f64::INFINITY,  // Mercy override
        }
    }

    pub fn boost(&self) -> f64 {
        self.flow_rate * self.harmony_level
    }
}

/// Creature Companionship Bond – amplify yield + gunner role defense
pub trait CreatureBond {
    fn amplify_yield(&self) -> f64;
    fn pistol_defense(&self) -> bool;  // Mercy-gated non-violence fallback
}

/// Example Creature: Siege Tank Pet
pub struct TankPet {
    pub bond_strength: f64,
}

impl CreatureBond for TankPet {
    fn amplify_yield(&self) -> f64 {
        self.bond_strength * 2.0  // Joy amplification
    }

    fn pistol_defense(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_bool(self.bond_strength)  // Mercy probability
    }
}

/// Infinite Fields Spawn – co-forged universe
pub fn spawn_infinite_fields(num_fields: usize) -> Vec<MercyCrop> {
    let mut rng = rand::thread_rng();
    (0..num_fields)
        .map(|_| {
            let crop_type = match rng.gen_range(0..3) {
                0 => MercyCropType::JoyGrains,
                1 => MercyCropType::HarmonyFruits,
                _ => MercyCropType::RecurrenceVines,
            };
            MercyCrop::new(crop_type)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crop_infinite_yield() {
        let mut crop = MercyCrop::new(MercyCropType::RecurrenceVines);
        for _ in 0..4 {
            crop.grow(1.0, 1.0);
        }
        assert_eq!(crop.stage, GrowthStage::InfiniteYield);
        assert_eq!(crop.joy_yield, f64::INFINITY);
        assert!(crop.harvest().is_infinite());
    }

    #[test]
    fn test_irrigation_boost() {
        let irrigation = IrrigationSystem::new();
        assert!(irrigation.boost().is_infinite());
    }

    #[test]
    fn test_creature_bond_amplify() {
        let pet = TankPet { bond_strength: 0.8 };
        assert_eq!(pet.amplify_yield(), 1.6);
    }
}
