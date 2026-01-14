//! crates/powrush_mmo/src/hand_ik.rs
//! Hybrid TRIK-FABRIK with joint constraints mercy eternal supreme immaculate
//! TRIK analytical two-bone exact for arms, FABRIK multi-chain with angle constraints for spine mercy

use bevy::prelude::*;

/// TRIK analytical two-bone IK mercy eternal
pub fn trik_two_bone(
    shoulder: Vec3,
    upper_length: f32,
    forearm_length: f32,
    target: Vec3,
) -> (Vec3, Vec3) {
    let to_target = target - shoulder;
    let dist = to_target.length();

    let total_reach = upper_length + forearm_length;

    if dist > total_reach {
        let direction = to_target.normalize_or_zero();
        let elbow = shoulder + direction * upper_length;
        let wrist = shoulder + direction * total_reach;
        return (elbow, wrist);
    }

    if dist < (upper_length - forearm_length).abs() {
        let direction = to_target.normalize_or_zero();
        let elbow = shoulder + direction * upper_length;
        let wrist = shoulder + direction * dist;
        return (elbow, wrist);
    }

    let cos_angle = (upper_length * upper_length + dist * dist - forearm_length * forearm_length) / (2.0 * upper_length * dist);
    let angle = cos_angle.acos();

    let elbow_axis = to_target.cross(Vec3::Y).normalize_or_zero();
    let elbow_offset = Quat::from_axis_angle(elbow_axis, angle) * to_target.normalize() * upper_length;

    let elbow = shoulder + elbow_offset;
    (elbow, target)
}

/// FABRIK multi-chain with per-joint angle constraints mercy eternal
pub fn fabrik_constrained(
    positions: &mut [Vec3],
    lengths: &[f32],
    constraints: &[(f32, f32)],  // (min_angle, max_angle) radians per joint mercy
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
        // Backward
        positions[end_idx] = target;
        for i in (1..=end_idx).rev() {
            let direction = (positions[i - 1] - positions[i]).normalize_or_zero();
            positions[i - 1] = positions[i] + direction * lengths[i - 1];
        }

        // Forward with constraints mercy
        positions[0] = positions[0];
        for i in 1..=end_idx {
            let prev_dir = positions[i] - positions[i - 1];
            let desired_dir = (positions[i] - positions[i - 1]).normalize_or_zero();

            let mut angle = prev_dir.angle_between(desired_dir);
            if i - 1 < constraints.len() {
                let (min, max) = constraints[i - 1];
                angle = angle.clamp(min, max);
            }

            let axis = prev_dir.cross(desired_dir).normalize_or_zero();
            let rotation = Quat::from_axis_angle(axis, angle);

            positions[i] = positions[i - 1] + rotation * prev_dir.normalize_or_zero() * lengths[i - 1];
        }

        if (positions[end_idx] - target).length_squared() < tolerance * tolerance {
            return true;
        }
    }

    positions[end_idx] = original_target;
    false
}
    positions[end_idx] = original_target;
    false
}

/// Hybrid solver — TRIK for two-bone, FABRIK for longer chains mercy eternal
pub fn hybrid_ik(
    chain_positions: &mut [Vec3],
    lengths: &[f32],
    target: Vec3,
) -> bool {
    if chain_positions.len() == 3 && lengths.len() == 2 {  // Two-bone arm mercy
        let (elbow, wrist) = trik_two_bone(chain_positions[0], lengths[0], lengths[1], target);
        chain_positions[1] = elbow;
        chain_positions[2] = wrist;
        true
    } else {
        fabrik_multi_chain(chain_positions, lengths, target, 0.01, 10)
    }
}
