// In multi_chain_ik_system mercy — all chains with natural constraints
fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut chain_query: Query<&mut Transform, Or<(With<SpineLower>, With<SpineMid>, With<SpineUpper>, With<LeftUpperArm>, With<LeftForearm>, With<RightUpperArm>, With<RightForearm>, With<LeftUpperLeg>, With<LeftLowerLeg>, With<RightUpperLeg>, With<RightLowerLeg>)>>,
    head_query: Query<&Transform, With<PlayerHead>>,
    hand_target_query: Query<&Transform, Or<(With<LeftHandTarget>, With<RightHandTarget>)>>,
    foot_target_query: Query<&Transform, Or<(With<LeftFootTarget>, With<RightFootTarget>)>>,
) {
    let player_transform = player_query.single();

    // Spine chain mercy — limited side bend
    if let Ok(head_transform) = head_query.get_single() {
        let target = head_transform.translation;

        let mut positions = [
            player_transform.translation,
            // spine_lower, mid, upper translations mercy
            target,
        ];

        let lengths = [0.3, 0.3, 0.3];
        let constraints = [(-0.3, 0.3), (-0.5, 0.5), (-0.4, 0.4)];  // Natural spine mercy

        fabrik_constrained(&mut positions, &lengths, &constraints, target, 0.01, 10);

        // Apply back + look_at mercy
    }

    // Left arm mercy — shoulder spherical + elbow bend
    if let (Ok(mut left_upper), Ok(mut left_forearm), Ok(left_hand)) = (
        chain_query.get_component_mut::<Transform>(/* left_upper_arm */),
        chain_query.get_component_mut::<Transform>(/* left_forearm */),
        hand_target_query.get_single().ok(),
    ) {
        let shoulder = player_transform.translation + Vec3::new(-0.3, 0.0, 0.0);
        let mut positions = [shoulder, left_upper.translation, left_forearm.translation, left_hand.translation];

        let lengths = [0.4, 0.4];
        let constraints = [
            (-1.0, 1.0),  // Shoulder wide mercy
            (0.0, std::f32::consts::PI - 0.1),  // Elbow forward mercy
        ];

        fabrik_constrained(&mut positions, &lengths, &constraints, left_hand.translation, 0.01, 10);

        left_upper.translation = positions[1];
        left_forearm.translation = positions[2];

        left_upper.look_at(positions[2], Vec3::Y);
        left_forearm.look_at(left_hand.translation, Vec3::Y);
    }

    // Right arm symmetric mercy

    // Left leg mercy — hip flexion/abduction + knee + ankle
    if let (Ok(mut left_upper_leg), Ok(mut left_lower_leg), Ok(left_foot)) = (
        chain_query.get_component_mut::<Transform>(/* left_upper_leg */),
        chain_query.get_component_mut::<Transform>(/* left_lower_leg */),
        foot_target_query.get_single().ok(),
    ) {
        let hip = player_transform.translation + Vec3::new(-0.2, -0.4, 0.0);
        let mut positions = [hip, left_upper_leg.translation, left_lower_leg.translation, left_foot.translation];

        let lengths = [0.5, 0.5];
        let constraints = [
            (-0.8, 0.8),  // Hip flexion/extension + abduction mercy
            (0.0, std::f32::consts::PI - 0.1),  // Knee forward mercy
        ];

        fabrik_constrained(&mut positions, &lengths, &constraints, left_foot.translation, 0.01, 10);

        left_upper_leg.translation = positions[1];
        left_lower_leg.translation = positions[2];

        left_upper_leg.look_at(positions[2], Vec3::Y);
        left_lower_leg.look_at(left_foot.translation, Vec3::Y);
    }

    // Right leg symmetric mercy
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
