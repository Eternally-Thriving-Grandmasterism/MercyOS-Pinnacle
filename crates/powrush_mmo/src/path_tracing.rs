//! crates/powrush_mmo/src/path_tracing.rs
//! Unbiased path tracing with adaptive sampling mercy eternal supreme immaculate
//! Variance-guided sample allocation from SVGF history + accumulation mercy

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use wgpu::util::DeviceExt;

#[derive(Resource)]
pub struct PathTracer {
    pub pipeline: ComputePipeline,
    pub bind_group_layout: BindGroupLayout,
    pub accumulation_texture: Texture,
    pub accumulation_view: TextureView,
    pub variance_texture: Texture,      // From SVGF mercy
    pub variance_view: TextureView,
    pub sample_count_buffer: Buffer,    // Per-pixel sample count mercy
}

pub fn setup_path_tracer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut images: ResMut<Assets<Image>>,
) {
    let shader = render_device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Adaptive Path Tracer Shader"),
        source: ShaderSource::Wgsl(include_str!("adaptive_path_tracer.wgsl").into()),
    });

    let bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Adaptive Path Tracer Bind Group Layout"),
        entries: &[
            // Voxel data, camera, accumulation, variance, sample_count mercy
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadWrite,
                    format: TextureFormat::Rgba32Float,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Adaptive Path Tracer Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("Adaptive Path Tracer Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "cs_main",
    });

    let size = Extent3d { width: 1920, height: 1080, depth_or_array_layers: 1 };

    let accumulation_texture = render_device.create_texture(&TextureDescriptor {
        label: Some("Path Tracer Accumulation"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let accumulation_view = accumulation_texture.create_view(&TextureViewDescriptor::default());

    let variance_texture = render_device.create_texture(&TextureDescriptor {
        label: Some("Variance Map"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::R32Float,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let variance_view = variance_texture.create_view(&TextureViewDescriptor::default());

    let sample_count_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Sample Count Buffer"),
        contents: &vec![4u32; (1920 * 1080) as usize * std::mem::size_of::<u32>()],
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    commands.insert_resource(PathTracer {
        pipeline,
        bind_group_layout,
