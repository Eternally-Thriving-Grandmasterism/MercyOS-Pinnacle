// Mercy Glow Shader — Positive Emotional Resonance Eternal
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let mercy_color = vec4<f32>(0.2, 0.8, 1.0, 0.5);
    return mercy_color + in.color;
}
