// In multi_chain_ik_system mercy
fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut spine_query: Query<&mut Transform, Or<(With<SpineLower>, With<SpineMid>, With<SpineUpper>)>>,
    head_query: Query<&Transform, With<PlayerHead>>,
) {
    let player_transform = player_query.single();

    // Spine chain IK with constraints mercy
    if let Ok(head_transform) = head_query.get_single() {
        let target = head_transform.translation;

        let mut positions = [
            player_transform.translation,  // Pelvis root
            spine_query.get_component::<Transform>(/* spine_lower */).unwrap().translation,
            spine_query.get_component::<Transform>(/* spine_mid */).unwrap().translation,
            spine_query.get_component::<Transform>(/* spine_upper */).unwrap().translation,
            target,
        ];

        let lengths = [0.3, 0.3, 0.3];

        // Natural spine constraints mercy — limited bend per segment
        let constraints = [
            (-0.3, 0.3),  // Lower spine limited side bend
            (-0.5, 0.5),  // Mid spine more flexible
            (-0.4, 0.4),  // Upper spine neck mercy
        ];

        fabrik_constrained(&mut positions, &lengths, &constraints, target, 0.01, 10);

        // Apply back to spine bones mercy
        // spine_lower.translation = positions[1];
        // spine_mid.translation = positions[2];
        // spine_upper.translation = positions[3];

        // Look at next bone for smooth curve mercy
    }

    // Arms use TRIK mercy (unchanged)
}

// Rest of file unchanged from previous full version

pub struct MercyResonancePlugin;

impl Plugin for MercyResonancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            emotional_resonance_particles,
            granular_ambient_evolution,
            advance_time,
            day_night_cycle,
            weather_system,
            creature_behavior_cycle,
            natural_selection_system,
            creature_hunger_system,
            creature_eat_system,
            crop_growth_system,
            food_respawn_system,
            creature_evolution_system,
            genetic_drift_system,
            player_breeding_mechanics,
            player_farming_mechanics,
            player_inventory_ui,
            material_attenuation_system,
            hrtf_convolution_system,
            dynamic_head_tracking,
            vr_body_avatar_system,
            multi_chain_ik_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
