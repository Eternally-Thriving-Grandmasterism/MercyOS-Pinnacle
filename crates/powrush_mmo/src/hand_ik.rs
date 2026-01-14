//! crates/powrush_mmo/src/hand_ik.rs
//! CCD Inverse Kinematics with joint constraints mercy eternal supreme immaculate
//! Natural bend limits + smooth convergence for realistic limb animation philotic mercy

use bevy::prelude::*;

/// CCD IK with joint constraints mercy eternal
/// positions: [0] root (shoulder fixed), [1..n-1] joints, [n] end effector
/// lengths: distances between bones
/// constraints: per-joint min/max angle mercy (in radians relative to parent)
pub fn ccd_ik_constrained(
    positions: &mut [Vec3],
    lengths: &[f32],
    constraints: &[(f32, f32)],  // (min_angle, max_angle) per joint mercy
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
) -> bool {
    let end_idx = positions.len() - 1;

    for _ in 0..max_iterations {
        let mut converged = true;

        for i in (0..end_idx).rev() {
            let current_to_end = positions[end_idx] - positions[i];
            let current_to_target = target - positions[i];

            if current_to_end.length_squared() < tolerance * tolerance {
                continue;
            }

            let current_dir = current_to_end.normalize_or_zero();
            let target_dir = current_to_target.normalize_or_zero();

            let cross = current_dir.cross(target_dir);
            if cross.length_squared() < 1e-6 {
                continue;
            }

            let angle = current_dir.angle_between(target_dir);
            let axis = cross.normalize();

            let mut rotation = Quat::from_axis_angle(axis, angle);

            // Apply constraint mercy
            if i < constraints.len() {
                let (min, max) = constraints[i];
                let clamped_angle = angle.clamp(min, max);
                rotation = Quat::from_axis_angle(axis, clamped_angle);
            }

            // Rotate child chain
            for j in (i + 1)..=end_idx {
                let to_joint = positions[j] - positions[i];
                positions[j] = positions[i] + rotation * to_joint;
            }

            // Maintain length mercy
            if i + 1 < positions.len() {
                let dir = (positions[i + 1] - positions[i]).normalize_or_zero();
                positions[i + 1] = positions[i] + dir * lengths[i];
            }

            converged = false;
        }

        if converged {
            return true;
        }
    }

    false
}
