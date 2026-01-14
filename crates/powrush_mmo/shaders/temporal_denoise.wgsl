// Temporal reprojection + variance clipping mercy eternal

@group(0) @binding(0) var current_color: texture_2d<f32>;
@group(0) @binding(1) var history_color: texture_2d<f32>;
@group(0) @binding(2) var motion_vectors: texture_2d<f32>;
@group(0) @binding(3) var normal_depth: texture_2d<f32>;
@group(0) @binding(4) var<storage, read_write> output: texture_storage_2d<rgba16float, write>;

@compute @workgroup_size(8, 8)
fn temporal_denoise(@builtin(global_invocation_id) gid: vec3<u32>) {
    // Reproject history using motion vectors mercy
    // Variance clipping + color clamping + blend mercy eternal
}
