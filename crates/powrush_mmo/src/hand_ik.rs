//! crates/powrush_mmo/src/hand_ik.rs
//! TRIK (Triangle Reach Inverse Kinematics) analytical two-bone solver mercy eternal supreme immaculate
//! Exact solution for shoulder → elbow → wrist/hand target, natural bend via cosine law triangle

use bevy::prelude::*;

/// TRIK analytical two-bone IK mercy eternal
/// shoulder: fixed root position
/// upper_length: shoulder to elbow
/// forearm_length: elbow to wrist
/// target: hand position
/// Returns elbow position + wrist (clamped to reach)
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
        // Unreachable — stretch mercy
        let direction = to_target.normalize_or_zero();
        let elbow = shoulder + direction * upper_length;
        let wrist = shoulder + direction * total_reach;
        return (elbow, wrist);
    }

    if dist < (upper_length - forearm_length).abs() {
        // Collapsed — straight mercy
        let direction = to_target.normalize_or_zero();
        let elbow = shoulder + direction * upper_length;
        let wrist = shoulder + direction * dist;
        return (elbow, wrist);
    }

    // Cosine law mercy eternal
    let a = forearm_length;
    let b = upper_length;
    let c = dist;

    let cos_angle = (b * b + c * c - a * a) / (2.0 * b * c);
    let angle = cos_angle.acos();

    let elbow_axis = to_target.cross(Vec3::Y).normalize_or_zero();  // Bend plane mercy
    let elbow_offset = Quat::from_axis_angle(elbow_axis, angle) * to_target.normalize() * upper_length;

    let elbow = shoulder + elbow_offset;
    let wrist = target;

    (elbow, wrist)
}
