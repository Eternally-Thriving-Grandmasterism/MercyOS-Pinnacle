// In multi_chain_ik_system mercy — arm chains with shoulder constraints
fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut arm_query: Query<&mut Transform, Or<(With<LeftUpperArm>, With<RightUpperArm>, With<LeftForearm>, With<RightForearm>)>>,
    hand_target_query: Query<&Transform, Or<(With<LeftHandTarget>, With<RightHandTarget>)>>,
) {
    let player_transform = player_query.single();

    // Left arm TRIK + shoulder constraints mercy
    if let (Ok(mut left_upper), Ok(mut left_forearm), Ok(left_hand)) = (
        arm_query.get_component_mut::<Transform>(/* left_upper_arm entity */),
        arm_query.get_component_mut::<Transform>(/* left_forearm entity */),
        hand_target_query.get_single().ok(),
    ) {
        let shoulder = player_transform.translation + Vec3::new(-0.3, 0.0, 0.0);
        let target = left_hand.translation;

        let (elbow, _) = trik_two_bone(shoulder, 0.4, 0.4, target);

        left_upper.translation = (shoulder + elbow) / 2.0;
        left_upper.look_at(elbow, Vec3::Y);

        left_forearm.translation = (elbow + target) / 2.0;
        left_forearm.look_at(target, Vec3::Y);

        // Apply shoulder spherical constraints mercy
        apply_shoulder_constraints(shoulder, &mut left_upper, player_transform.forward(), player_transform.up());
    }

    // Right arm symmetric mercy

    // Spine and legs unchanged mercy
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
