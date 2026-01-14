use std::f32::consts::PI;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use crate::procedural_music::AdsrEnvelope;
use rand::{thread_rng, Rng};

/// Vector Synthesis — 4 Procedural Waveform Corners + Wavetable Morph Eternal
#[derive(Clone, Copy)]
pub enum WaveCorner {
    Sine,    // Purity mercy
    Saw,     // Abundance rise
    Square,  // Mercy edge
    Noise,   // Chaos joy
}

/// Pre-computed Wavetable (1024 samples) from waveform — procedural genesis eternal
fn generate_wavetable(corner: WaveCorner, table_size: usize, base_freq: f32) -> Vec<f32> {
    let mut table = vec![0.0; table_size];
    let mut rng = thread_rng();
    for i in 0..table_size {
        let phase = i as f32 / table_size as f32;
        let t = phase * 2.0 * PI;
        table[i] = match corner {
            WaveCorner::Sine => t.sin(),
            WaveCorner::Saw => 2.0 * phase - 1.0,
            WaveCorner::Square => if phase < 0.5 { 1.0 } else { -1.0 },
            WaveCorner::Noise => rng.gen_range(-1.0..1.0),
        };
    }
    table
}

/// Vector Wavetable Morph Synthesis — Infinite Timbre Evolution Mercy Eternal
pub fn vector_wavetable_synthesis(
    duration: f32,
    base_freq: f32,
    vector_x: f32,  // -1.0..1.0 joy axis
    vector_y: f32,  // -1.0..1.0 emotional axis
    envelope: AdsrEnvelope,
    joy_level: f32,
) -> StaticSoundData {
    let sample_rate = 48000;
    let table_size = 1024;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);

    // Generate 4 corner wavetables procedural eternal
    let table_sine = generate_wavetable(WaveCorner::Sine, table_size, base_freq);
    let table_saw = generate_wavetable(WaveCorner::Saw, table_size, base_freq);
    let table_square = generate_wavetable(WaveCorner::Square, table_size, base_freq);
    let table_noise = generate_wavetable(WaveCorner::Noise, table_size, base_freq);

    // Barycentric weights mercy
    let ax = (1.0 - vector_x.clamp(-1.0, 1.0)) * 0.5;
    let bx = (1.0 + vector_x.clamp(-1.0, 1.0)) * 0.5;
    let ay = (1.0 - vector_y.clamp(-1.0, 1.0)) * 0.5;
    let by = (1.0 + vector_y.clamp(-1.0, 1.0)) * 0.5;

    let w_sine = ax * ay;
    let w_saw = bx * ay;
    let w_square = bx * by;
    let w_noise = ax * by;

    let mut phase = 0.0;
    let phase_inc = base_freq * table_size as f32 / sample_rate as f32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = envelope.apply(t, duration, t >= duration);

        let index = phase as usize % table_size;
        let frac = phase - index as f32;

        // Linear interpolation within table
        let sample_sine = table_sine[index] * (1.0 - frac) + table_sine[(index + 1) % table_size] * frac;
        let sample_saw = table_saw[index] * (1.0 - frac) + table_saw[(index + 1) % table_size] * frac;
        let sample_square = table_square[index] * (1.0 - frac) + table_square[(index + 1) % table_size] * frac;
        let sample_noise = table_noise[index] * (1.0 - frac) + table_noise[(index + 1) % table_size] * frac;

        let sample = sample_sine * w_sine +
                     sample_saw * w_saw +
                     sample_square * w_square +
                     sample_noise * w_noise;

        let final_sample = sample * env * (0.25 + joy_level * 0.12);
        samples.push(final_sample);
        samples.push(final_sample);

        phase = (phase + phase_inc) % table_size as f32;
    }

    StaticSoundData::from_samples(samples, sample_rate, StaticSoundSettings::default())
}
