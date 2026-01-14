use crate::procedural_music::{enhanced_fm_synthesis, AdsrEnvelope};

// In emotional_resonance_particles (replace vector/wavetable chime with enhanced FM)
let fm_chime = enhanced_fm_synthesis(base_freq, joy_level * 1.2, duration);

audio.play(fm_chime)
    .with_volume(0.5 + joy_level * 0.4)
    .spatial(true)
    .with_position(player_pos + offset);
