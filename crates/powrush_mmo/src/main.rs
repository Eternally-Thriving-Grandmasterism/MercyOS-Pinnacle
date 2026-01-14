// In multi_chain_ik_system mercy — add leg constraints
fn multi_chain_ik_system(
    player_query: Query<&Transform, With<Player>>,
    mut leg_query: Query<&mut Transform, Or<(With<LeftUpperLeg>, With<LeftLowerLeg>, With<RightUpperLeg>, With<RightLowerLeg>)>>,
    foot_target_query: Query<&Transform, Or<(With<LeftFootTarget>, With<RightFootTarget>)>>,
) {
    let player_transform = player_query.single();

    // Left leg IK with knee constraint mercy
    if let (Ok(mut left_upper_leg), Ok(mut left_lower_leg), Ok(left_foot)) = (
        leg_query.get_component_mut::<Transform>(/* left_upper_leg entity */),
        leg_query.get_component_mut::<Transform>(/* left_lower_leg entity */),
        foot_target_query.get_single().ok(),
    ) {
        let hip = player_transform.translation + Vec3::new(-0.2, -0.4, 0.0);
        let mut positions = [
            hip,
            left_upper_leg.translation,
            left_lower_leg.translation,
            left_foot.translation,
        ];

        let lengths = [0.5, 0.5];

        // Knee forward bend only mercy — 0.0 to PI
        let constraints = [(0.0, std::f32::consts::PI)];

        fabrik_constrained(&mut positions, &lengths, &constraints, left_foot.translation, 0.01, 10);

        left_upper_leg.translation = positions[1];
        left_lower_leg.translation = positions[2];

        left_upper_leg.look_at(positions[2], Vec3::Y);
        left_lower_leg.look_at(left_foot.translation, Vec3::Y);
    }

    // Right leg symmetric mercy

    // Other chains unchanged
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
