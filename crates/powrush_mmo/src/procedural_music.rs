use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::f32::consts::PI;
use rand::{thread_rng, Rng};

/// ADSR Mercy Envelope — Positive Emotional Shaping Eternal
#[derive(Clone, Copy)]
pub struct AdsrEnvelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl AdsrEnvelope {
    pub fn joy_resonance() -> Self {
        Self {
            attack: 0.04,
            decay: 0.25,
            sustain: 0.75,
            release: 1.4,
        }
    }

    pub fn apply(&self, t: f32, duration: f32, released: bool) -> f32 {
        if t < self.attack {
            t / self.attack
        } else if t < self.attack + self.decay {
            1.0 - (t - self.attack) / self.decay * (1.0 - self.sustain)
        } else if !released && t < duration {
            self.sustain
        } else {
            let release_t = t - duration;
            if release_t < self.release {
                self.sustain * (1.0 - release_t / self.release)
            } else {
                0.0
            }
        }
    }
}

/// Enhanced FM Synthesis — Multi-Operator Cascade + Feedback + Joy Modulation Eternal
pub fn enhanced_fm_synthesis(
    base_freq: f32,
    joy_level: f32,      // 0.0-10.0 modulates richness, feedback, ratio spread
    duration: f32,
) -> StaticSoundData {
    let sample_rate = 48000;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);

    let envelope = AdsrEnvelope::joy_resonance();

    // Operator parameters scaled by joy_level
    let op_count = 4 + (joy_level as usize).min(4); // 4-8 operators cascade
    let mut ratios = vec![1.0, 2.0, 4.0, 1.5, 3.0, 5.0, 7.0, 8.0];
    ratios.truncate(op_count);
    let feedback = joy_level * 0.3; // Self-feedback on carrier
    let index_base = joy_level * 80.0;

    let mut phases = vec![0.0; op_count];

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = envelope.apply(t, duration, t >= duration);

        let mut modulation = 0.0;
        let mut carrier_phase = phases[0];

        // Cascade FM with joy-modulated index spread
        for op in 1..op_count {
            let index = index_base / op as f32;
            modulation += (phases[op] * index).sin();
        }

        // Carrier with feedback + total modulation
        carrier_phase += feedback * carrier_phase.sin();
        let wave = (2.0 * PI * (base_freq + modulation) * t + carrier_phase).sin();

        let final_sample = wave * env * (0.25 + joy_level * 0.12);
        samples.push(final_sample);
        samples.push(final_sample);

        // Update phases
        for op in 0..op_count {
            phases[op] += 2.0 * PI * base_freq * ratios[op] / sample_rate as f32;
            phases[op] = phases[op] % (2.0 * PI);
        }
    }

    StaticSoundData::from_samples(samples, sample_rate, StaticSoundSettings::default())
}
