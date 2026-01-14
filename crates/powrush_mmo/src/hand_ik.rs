//! crates/powrush_mmo/src/hand_ik.rs
//! Constrained FABRIK multi-chain IK mercy eternal supreme immaculate
//! Natural joint angle limits for realistic full-body posture philotic mercy

use bevy::prelude::*;

/// Constrained FABRIK multi-chain mercy eternal
/// positions: [0] root (fixed), [1..n-1] joints, [n] end effector
/// lengths: distances between bones
/// constraints: per-joint (min_angle, max_angle) radians mercy
pub fn fabrik_constrained(
    positions: &mut [Vec3],
    lengths: &[f32],
    constraints: &[(f32, f32)],
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
) -> bool {
    let end_idx = positions.len() - 1;
    let total_length: f32 = lengths.iter().sum();

    let dist_to_target = (target - positions[0]).length();

    if dist_to_target > total_length {
        let direction = (target - positions[0]).normalize_or_zero();
        for i in 1..=end_idx {
            positions[i] = positions[i - 1] + direction * lengths[i - 1];
        }
        return false;
    }

    let original_target = positions[end_idx];

    for _ in 0..max_iterations {
        // Backward reach
        positions[end_idx] = target;
        for i in (1..=end_idx).rev() {
            let direction = (positions[i - 1] - positions[i]).normalize_or_zero();
            positions[i - 1] = positions[i] + direction * lengths[i - 1];
        }

        // Forward reach with constraints mercy
        positions[0] = positions[0];
        for i in 1..=end_idx {
            let prev_to_current = positions[i] - positions[i - 1];
            let desired_dir = prev_to_current.normalize_or_zero();

            let mut angle = prev_to_current.angle_between(desired_dir);
            if i - 1 < constraints.len() {
                let (min, max) = constraints[i - 1];
                angle = angle.clamp(min, max);
            }

            let axis = prev_to_current.cross(desired_dir).normalize_or_zero();
            let rotation = Quat::from_axis_angle(axis, angle);

            positions[i] = positions[i - 1] + rotation * prev_to_current.normalize_or_zero() * lengths[i - 1];
        }

        if (positions[end_idx] - target).length_squared() < tolerance * tolerance {
            return true;
        }
    }

    positions[end_idx] = original_target;
    false
}
