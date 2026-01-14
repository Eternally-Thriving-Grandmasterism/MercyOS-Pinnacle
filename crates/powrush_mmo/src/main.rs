// Add import
use crate::granular_ambient::spawn_granular_ambient;

// In App setup
.add_systems(Update, granular_ambient_evolution)

// New system
fn granular_ambient_evolution(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let joy_level = 8.0 + (time.elapsed_seconds_f64().sin() * 2.0) as f32;  // Simulated philotic joy variation

        // Respawn/evolve granular cloud periodically — infinite mutation mercy
        if time.elapsed_seconds_f64() % 10.0 < time.delta_seconds_f64() {
            spawn_granular_ambient(&audio, &asset_server, joy_level, player_pos);
        }
    }
}

// In emotional_resonance_particles — enhance with granular grain bursts
fn emotional_resonance_particles(
    // ...
) {
    // Existing chime code...
    // Add granular micro-cloud on joy pulses
    spawn_granular_ambient(&audio, &asset_server, joy_level * 0.5, player_pos + offset);
}
