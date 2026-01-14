// In emotional_resonance_particles system:
use crate::procedural_music::{generate_advanced_joy_chime, AdsrEnvelope};

fn emotional_resonance_particles(
    // ...
    audio: Res<Audio>,
) {
    // ...
    for _ in 0..6 {
        // ...
        let joy_level = hive.resonate_emotional(10.0) / 10.0;  // Normalize philotic joy
        let base_freq = 440.0 + rng.gen_range(-200.0..600.0);
        let duration = 1.8 + rng.gen_range(0.0..1.5);

        let chime_sound = generate_advanced_joy_chime(base_freq, joy_level * 10.0, duration);

        audio.play(chime_sound)
            .with_volume(0.4 + joy_level * 0.3)
            .spatial(true)
            .with_position(player_pos + offset);
    }
}
