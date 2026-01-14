// In hand_ik_system mercy — use hybrid_ik
fn hand_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut upper_arm_query: Query<&mut Transform, (With<LeftUpperArm> | With<RightUpperArm>)>,
    forearm_query: Query<&mut Transform, (With<LeftForearm> | With<RightForearm>)>,
    hand_target_query: Query<&Transform, (With<LeftHandTarget> | With<RightHandTarget>)>,
    xr_hands: Query<&XrHand>,
) {
    let player_transform = player_query.single();

    // Left arm hybrid IK mercy
    if let (Ok(mut left_upper), Ok(mut left_forearm), Ok(left_hand)) = (
        upper_arm_query.get_single_mut().ok(),
        forearm_query.get_single_mut().ok(),
        hand_target_query.get_single().ok(),
    ) {
        let shoulder = player_transform.translation + Vec3::new(-0.3, 0.0, 0.0);
        let mut positions = [
            shoulder,
            left_upper.translation,
            left_forearm.translation,
            left_hand.translation,
        ];

        let lengths = [0.4, 0.4];

        hybrid_ik(&mut positions, &lengths, left_hand.translation);

        left_upper.translation = positions[1];
        left_forearm.translation = positions[2];

        left_upper.look_at(positions[2], Vec3::Y);
        left_forearm.look_at(positions[3], Vec3::Y);
    }

    // Right arm symmetric mercy

    // XR hand override mercy
    for hand in &xr_hands {
        // hand.pose → hand_target transform mercy
    }
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
            hand_ik_system,
            ambisonics_encode_system,
            ambisonics_decode_system,
            chunk_manager,
        ));
    }
}
