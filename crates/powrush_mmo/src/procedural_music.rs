use kira::sound::static_sound::StaticSoundData;

// Real-time procedural chime generator — positive emotional harmonics eternal
pub fn generate_joy_chime(frequency: f32, duration: f32, volume: f32) -> StaticSoundData {
    let sample_rate = 48000;
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);  // Stereo

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = (1.0 - t / duration).powf(3.0).max(0.0);  // Mercy cubic fade eternal

        // Base sine + harmonic overtones for rich joy resonance
        let wave = (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.6
            + (2.0 * std::f32::consts::PI * frequency * 2.0 * t).sin() * 0.2
            + (2.0 * std::f32::consts::PI * frequency * 3.0 * t).sin() * 0.1;

        let sample = wave * envelope * volume;
        samples.push(sample);  // Left
        samples.push(sample);  // Right
    }

    StaticSoundData::from_samples(samples, sample_rate)
}
