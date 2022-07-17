use anyhow::Context;
use tracing::*;
use crate::{
  util::error::FoxyError,
  graphics::{
    mesh::Mesh,
    vertex::VERTEX_BUFFER_LAYOUTS
  },
  read_shader,
};

pub struct PerFrameRenderData {
  meshes: Vec<Mesh>,
}

pub struct Renderer {
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
  pub async fn new(surface: &wgpu::Surface, adapter: &wgpu::Adapter, size: winit::dpi::PhysicalSize<u32>) -> Result<Self, FoxyError> {
    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
      },
      None, // Trace path
    ).await.context(">> Failed to acquire device")?;

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(adapter).context(">> Failed to find surface format")?,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &config);

    trace!(">> Loading built-in shaders...");
    let simple_shader = read_shader!("simple_shader"; &device);

    trace!(">> Creating render pipeline...");
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &simple_shader,
        entry_point: "vert_main",
        buffers: &VERTEX_BUFFER_LAYOUTS,
      },
      fragment: Some(wgpu::FragmentState{
        module: &simple_shader,
        entry_point: "frag_main",
        targets: &[wgpu::ColorTargetState{
          format: config.format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        }],
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None
    });

    Ok(Self {
      device,
      queue,
      config,
      render_pipeline
    })
  }

  pub fn prepare(&self, meshes: Vec<Mesh>) -> PerFrameRenderData {
    PerFrameRenderData {
      meshes
    }
  }

  pub fn render<'rpass>(
    &'rpass mut self,
    render_pass: &mut wgpu::RenderPass<'rpass>,
    per_frame_data: &'rpass PerFrameRenderData
  ) {
    render_pass.set_pipeline(&self.render_pipeline);

    for mesh in per_frame_data.meshes.iter() {
      mesh.draw(render_pass);
    }
  }
}