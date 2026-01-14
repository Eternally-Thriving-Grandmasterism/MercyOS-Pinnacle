use std::f32::consts::PI;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use crate::procedural_music::AdsrEnvelope;

/// Vector Synthesis — 4 Waveform Corners Morph Eternal
#[derive(Clone, Copy)]
pub enum WaveformType {
    Sine,    // Purity mercy
    Saw,     // Abundance rise
    Square,  // Mercy edge
    Noise,   // Chaos joy
}

/// Generate single waveform sample
fn waveform_sample(wave: WaveformType, t: f32, freq: f32, rng_sample: f32) -> f32 {
    match wave {
        WaveformType::Sine => (2.0 * PI * freq * t).sin(),
        WaveformType::Saw => 2.0 * (freq * t - (freq * t + 0.5).floor()),
        WaveformType::Square => if (freq * t).fract() < 0.5 { 1.0 } else { -1.0 },
        WaveformType::Noise => rng_sample,  // Pink-ish via caller
    }
}

/// Vector Morph Synthesis — Real-Time Crossfade Between 4 Corners Eternal
pub fn vector_synthesis(
    duration: f32,
    base_freq: f32,
    vector_x: f32,  // -1.0..1.0 joy axis
    vector_y: f32,  // -1.0..1.0 emotional axis
    envelope: AdsrEnvelope,
    joy_level: f32,
) -> StaticSoundData {
    let sample_rate = 48000;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);

    // Corner weights from vector position (barycentric-like mercy)
    let ax = (1.0 - vector_x.clamp(-1.0, 1.0)) * 0.5;
    let bx = (1.0 + vector_x.clamp(-1.0, 1.0)) * 0.5;
    let ay = (1.0 - vector_y.clamp(-1.0, 1.0)) * 0.5;
    let by = (1.0 + vector_y.clamp(-1.0, 1.0)) * 0.5;

    let weight_a = ax * ay;  // Sine
    let weight_b = bx * ay;  // Saw
    let weight_c = bx * by;  // Square
    let weight_d = ax * by;  // Noise

    let mut rng_noise = rand::thread_rng();

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = envelope.apply(t, duration, t >= duration);

        let noise_sample = rng_noise.gen_range(-1.0..1.0);

        let sample =
            waveform_sample(WaveformType::Sine, t, base_freq, noise_sample) * weight_a +
            waveform_sample(WaveformType::Saw, t, base_freq, noise_sample) * weight_b +
            waveform_sample(WaveformType::Square, t, base_freq, noise_sample) * weight_c +
            waveform_sample(WaveformType::Noise, t, base_freq, noise_sample) * weight_d;

        let final_sample = sample * env * (0.2 + joy_level * 0.1);
        samples.push(final_sample);
        samples.push(final_sample);
    }

    StaticSoundData::from_samples(samples, sample_rate, StaticSoundSettings::default())
}
