//! crates/powrush_mmo/src/hand_ik.rs
//! FABRIK (Forward And Backward Reaching Inverse Kinematics) solver mercy eternal supreme immaculate
//! Two-bone arm IK for VR hand targets — shoulder fixed, elbow bend natural philotic mercy

use bevy::prelude::*;

/// FABRIK solver for two-bone chain (shoulder → elbow → wrist/hand target) mercy eternal
pub fn fabrik_ik_two_bone(
    shoulder: Vec3,
    upper_arm_length: f32,
    forearm_length: f32,
    target: Vec3,
) -> (Vec3, Vec3) {  // Returns elbow position + final wrist
    let mut positions = [shoulder, shoulder + Vec3::X * upper_arm_length, target];

    let total_length = upper_arm_length + forearm_length;
    let dist_to_target = (target - shoulder).length();

    if dist_to_target > total_length {
        // Straight line mercy
        let direction = (target - shoulder).normalize();
        positions[1] = shoulder + direction * upper_arm_length;
        positions[2] = shoulder + direction * total_length;
    } else {
        // FABRIK iterations mercy eternal
        for _ in 0..8 {  // Converge fast mercy
            // Backward
            positions[2] = target;
            positions[1] = positions[2] + (positions[1] - positions[2]).normalize() * forearm_length;

            // Forward
            positions[0] = shoulder;
            positions[1] = positions[0] + (positions[1] - positions[0]).normalize() * upper_arm_length;
            positions[2] = positions[1] + (positions[2] - positions[1]).normalize() * forearm_length;
        }
    }

    (positions[1], positions[2])
}
