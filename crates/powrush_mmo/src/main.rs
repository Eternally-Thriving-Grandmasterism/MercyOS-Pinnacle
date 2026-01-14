// Add import
use crate::vector_synthesis::vector_wavetable_synthesis;

// In emotional_resonance_particles
let vector_x = (time.elapsed_seconds_f64() * 0.5).sin() as f32 * joy_level;
let vector_y = (time.elapsed_seconds_f64() * 0.3).cos() as f32 * joy_level;

let wavetable_chime = vector_wavetable_synthesis(duration, base_freq, vector_x, vector_y, AdsrEnvelope::joy_resonance(), joy_level);

audio.play(wavetable_chime)
    .with_volume(0.45 + joy_level * 0.35)
    .spatial(true)
    .with_position(player_pos + offset);
