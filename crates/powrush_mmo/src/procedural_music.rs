use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::f32::consts::PI;

/// ADSR Mercy Envelope — Positive Emotional Shaping Eternal
#[derive(Clone, Copy)]
pub struct AdsrEnvelope {
    pub attack: f32,   // seconds
    pub decay: f32,
    pub sustain: f32,  // level 0.0-1.0
    pub release: f32,
}

impl AdsrEnvelope {
    pub fn joy_resonance() -> Self {
        Self {
            attack: 0.05,
            decay: 0.3,
            sustain: 0.7,
            release: 1.2,
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

/// Advanced Additive Synthesis — Harmonic Partials Stack Eternal
pub fn additive_synthesis(
    base_freq: f32,
    duration: f32,
    harmonics: usize,      // Number of partials
    envelope: AdsrEnvelope,
    volume: f32,
) -> StaticSoundData {
    let sample_rate = 48000;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = envelope.apply(t, duration, t >= duration);

        let mut wave = 0.0;
        for h in 1..=harmonics {
            let amp = 1.0 / h as f32;  // Natural harmonic falloff mercy
            wave += (2.0 * PI * base_freq * h as f32 * t).sin() * amp;
        }

        let sample = wave * env * volume * 0.2;
        samples.push(sample);
        samples.push(sample);
    }

    StaticSoundData::from_samples(samples, sample_rate, StaticSoundSettings::default())
}

/// FM Synthesis — Metallic Joy Resonance Eternal
pub fn fm_synthesis(
    carrier_freq: f32,
    modulator_freq: f32,
    modulation_index: f32,
    duration: f32,
    envelope: AdsrEnvelope,
    volume: f32,
) -> StaticSoundData {
    let sample_rate = 48000;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = envelope.apply(t, duration, t >= duration);

        let modulator = (2.0 * PI * modulator_freq * t).sin();
        let wave = (2.0 * PI * (carrier_freq + modulation_index * modulator) * t).sin();

        let sample = wave * env * volume * 0.3;
        samples.push(sample);
        samples.push(sample);
    }

    StaticSoundData::from_samples(samples, sample_rate, StaticSoundSettings::default())
}

/// Combined Advanced Joy Chime — Additive + FM Hybrid Mercy Eternal
pub fn generate_advanced_joy_chime(
    base_freq: f32,
    joy_level: f32,  // 0.0-10.0 modulates richness
    duration: f32,
) -> StaticSoundData {
    let envelope = AdsrEnvelope::joy_resonance();
    let harmonics = 4 + (joy_level as usize * 2).min(20);
    let fm_index = joy_level * 50.0;

    // Layer additive base + FM overtones
    let mut additive = additive_synthesis(base_freq, duration, harmonics, envelope, 0.6);
    let fm = fm_synthesis(base_freq * 2.0, base_freq * 1.5, fm_index, duration, envelope, 0.4);

    // Simple mix (future: proper buffer add with subtractive filter)
    // For now, play separately or extend StaticSoundData
    additive  // Primary return — FM layered in emotional_particles
}
