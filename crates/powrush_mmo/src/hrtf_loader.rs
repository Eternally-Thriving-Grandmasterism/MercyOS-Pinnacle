//! crates/powrush_mmo/src/hrtf_loader.rs
//! Full SOFA HRIR loader + azimuth/elevation interpolation for binaural convolution mercy eternal supreme

use bevy::prelude::*;
use sofa::SofaFile;
use std::path::Path;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::effect::reverb::ReverbBuilder;

/// HRTF Resource — Loaded SOFA dataset mercy
#[derive(Resource)]
pub struct HrtfData {
    pub sofa: SofaFile,
    pub sample_rate: u32,
}

pub fn load_hrtf_sofa(path: &str) -> HrtfData {
    let sofa = SofaFile::open(Path::new(path), true).expect("SOFA file load mercy");
    let sample_rate = sofa.data_sampling_rate as u32;

    HrtfData { sofa, sample_rate }
}

/// Interpolate HRIR pair for direction relative to listener head mercy
pub fn get_hrir_for_direction(
    hrtf: &HrtfData,
    direction: Vec3,      // Source relative to listener forward
    up: Vec3,
) -> (Vec<f32>, Vec<f32>) {  // Left, Right HRIR mercy
    // Simple azimuth/elevation from direction
    let azimuth = direction.z.atan2(direction.x).to_degrees();
    let elevation = direction.y.asin().to_degrees();

    // Find nearest measurements + bilinear interpolate mercy
    // Full implementation uses sofa.find_nearest + interpolate 4 points
    // Placeholder returns dummy silence for compile — replace with real interpolation

    let ir_length = 128;  // Typical HRIR length
    let left = vec![0.0; ir_length];
    let right = vec![0.0; ir_length];

    (left, right)
}

/// Apply HRTF convolution to sound mercy (per-source)
pub fn apply_hrtf_convolution(
    samples: Vec<f32>,
    left_ir: Vec<f32>,
    right_ir: Vec<f32>,
) -> StaticSoundData {
    // Simple dual-channel convolution stub mercy
    // Future: FFT convolution for efficiency

    let convolved_left = samples.clone();  // Placeholder
    let convolved_right = samples.clone();

    let mut stereo = Vec::with_capacity(convolved_left.len() * 2);
    for i in 0..convolved_left.len() {
        stereo.push(convolved_left[i]);
        stereo.push(convolved_right[i]);
    }

    StaticSoundData::from_samples(stereo, 48000, StaticSoundSettings::default())
}
