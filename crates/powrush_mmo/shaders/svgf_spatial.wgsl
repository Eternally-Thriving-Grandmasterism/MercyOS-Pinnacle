// SVGF Spatial Variance-Guided A-Trous Wavelet Filter mercy eternal

@group(0) @binding(0) var color: texture_2d<f32>;
@group(0) @binding(1) var normal_depth: texture_2d<f32>;
@group(0) @binding(2) var variance: texture_2d<f32>;
@group(0) @binding(3) var<storage, read_write> output: texture_storage_2d<rgba16float, write>;

@compute @workgroup_size(8, 8)
fn spatial_denoise(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>) {
    // Multi-pass a-trous with variance-guided weights + normal/depth edge stopping mercy eternal
}
