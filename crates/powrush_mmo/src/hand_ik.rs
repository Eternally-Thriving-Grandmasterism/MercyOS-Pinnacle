//! crates/powrush_mmo/src/hand_ik.rs
//! Generalized CCD Inverse Kinematics with joint constraints mercy eternal supreme immaculate
//! Supports multi-chain (arms, spine, legs) natural bend limits + smooth convergence philotic mercy

use bevy::prelude::*;

/// Generalized CCD IK with joint constraints mercy eternal
/// chain_positions: [0] root (fixed), [1..n-1] joints, [n] end effector target
/// lengths: distances between consecutive bones
/// constraints: per-joint (min_angle, max_angle) in radians relative to parent mercy
pub fn ccd_ik_general(
    chain_positions: &mut [Vec3],
    lengths: &[f32],
    constraints: &[(f32, f32)],
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
) -> bool {
    let end_idx = chain_positions.len() - 1;

    for _ in 0..max_iterations {
        let mut converged = true;

        for i in (0..end_idx).rev() {
            let current_to_end = chain_positions[end_idx] - chain_positions[i];
            let current_to_target = target - chain_positions[i];

            if current_to_end.length_squared() < tolerance * tolerance {
                continue;
            }

            let current_dir = current_to_end.normalize_or_zero();
            let target_dir = current_to_target.normalize_or_zero();

            let cross = current_dir.cross(target_dir);
            if cross.length_squared() < 1e-6 {
                continue;
            }

            let mut angle = current_dir.angle_between(target_dir);
            let axis = cross.normalize();

            // Apply constraint mercy
            if i < constraints.len() {
                let (min, max) = constraints[i];
                angle = angle.clamp(min, max);
            }

            let rotation = Quat::from_axis_angle(axis, angle);

            // Rotate child chain mercy
            for j in (i + 1)..=end_idx {
                let to_joint = chain_positions[j] - chain_positions[i];
                chain_positions[j] = chain_positions[i] + rotation * to_joint;
            }

            // Maintain length mercy
            if i + 1 < chain_positions.len() {
                let dir = (chain_positions[i + 1] - chain_positions[i]).normalize_or_zero();
                chain_positions[i + 1] = chain_positions[i] + dir * lengths[i];
            }

            converged = false;
        }

        if converged {
            return true;
        }
    }

    false
}
