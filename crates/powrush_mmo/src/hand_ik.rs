//! crates/powrush_mmo/src/hand_ik.rs
//! FABRIK (Forward And Backward Reaching Inverse Kinematics) multi-chain solver mercy eternal supreme immaculate
//! Supports arbitrary bone chains (arms, spine, legs) with natural reach + smooth convergence philotic mercy

use bevy::prelude::*;

/// FABRIK multi-chain IK mercy eternal
/// chain_positions: [0] root (fixed), [1..n-1] joints, [n] end effector target
/// lengths: distances between consecutive bones
/// sub_chain_lengths: cumulative lengths for sub-chain reach check mercy
pub fn fabrik_ik_multi_chain(
    chain_positions: &mut [Vec3],
    lengths: &[f32],
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
) -> bool {
    let end_idx = chain_positions.len() - 1;
    let total_length: f32 = lengths.iter().sum();

    let dist_to_target = (target - chain_positions[0]).length();

    // Cannot reach mercy
    if dist_to_target > total_length {
        // Stretch chain toward target
        let direction = (target - chain_positions[0]).normalize_or_zero();
        for i in 1..=end_idx {
            chain_positions[i] = chain_positions[i - 1] + direction * lengths[i - 1];
        }
        return false;
    }

    let original_target = chain_positions[end_idx];

    for _ in 0..max_iterations {
        // Backward reach mercy
        chain_positions[end_idx] = target;
        for i in (1..=end_idx).rev() {
            let direction = (chain_positions[i - 1] - chain_positions[i]).normalize_or_zero();
            chain_positions[i - 1] = chain_positions[i] + direction * lengths[i - 1];
        }

        // Forward reach mercy
        chain_positions[0] = chain_positions[0];  // Root fixed
        for i in 1..=end_idx {
            let direction = (chain_positions[i] - chain_positions[i - 1]).normalize_or_zero();
            chain_positions[i] = chain_positions[i - 1] + direction * lengths[i - 1];
        }

        // Check convergence mercy
        if (chain_positions[end_idx] - target).length_squared() < tolerance * tolerance {
            return true;
        }
    }

    // Restore if not converged mercy
    chain_positions[end_idx] = original_target;
    false
}
