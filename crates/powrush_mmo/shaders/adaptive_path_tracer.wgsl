// Adaptive path tracer with variance-guided sampling mercy eternal

@group(0) @binding(0) var<storage, read> voxel_data: array<u32>;
@group(0) @binding(1) var accumulation_texture: texture_2d<f32>;
@group(0) @binding(2) var<storage, read_write> output_texture: texture_storage_2d<rgba32float, write>;
@group(0) @binding(3) var variance_map: texture_2d<f32>;
@group(0) @binding(4) var<storage, read> sample_counts: array<u32>;

struct Camera {
    position: vec3<f32>,
    forward: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>,
};

@group(0) @binding(5) var<uniform> camera: Camera;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let resolution = textureDimensions(output_texture);
    if (gid.x >= resolution.x || gid.y >= resolution.y) {
        return;
    }

    let pixel_idx = gid.y * resolution.x + gid.x;
    let sample_count = sample_counts[pixel_idx];
    let variance = textureLoad(variance_map, gid.xy, 0).r;

    // Adaptive samples mercy — high variance = more samples
    let target_samples = if variance > 0.1 { 64u } else if variance > 0.01 { 16u } else { 4u };

    // Path tracing loop mercy eternal
    var color = vec3<f32>(0.0);
    for (var s = 0u; s < sample_count; s++) {
        // Primary ray + bounces mercy
        color += trace_ray(gid.xy, s);
    }
    color /= f32(sample_count);

    let prev = textureLoad(accumulation_texture, gid.xy, 0).rgb;
    let accumulated = mix(prev, color, 1.0 / f32(sample_count + 1));

    textureStore(output_texture, gid.xy, vec4<f32>(accumulated, 1.0));
}
