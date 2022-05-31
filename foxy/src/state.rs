use crate::{
  util::{
    log::*,
    error::FoxyError,
    //color::RGBA
  },
  mesh::*,
  vertex::*,
};
use anyhow::Context;
use wgpu::Instance;
use winit::{
  event::*,
  window::Window,
};
use glam::*;
use rgb::{RGBA};

pub struct State {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub render_pipeline: wgpu::RenderPipeline,
  pub size: winit::dpi::PhysicalSize<u32>,
  meshes: Vec<Mesh>,
}

impl State {
  pub async fn new(window: &Window) -> Result<Self, FoxyError> {
    debug!("Intializing render system");
    let size = window.inner_size();
    let instance = Instance::new(wgpu::Backends::VULKAN);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).await.context("Failed to acquire adapter")?;

    info!("Supported features: {:?}", adapter.features());

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
      },
      None, // Trace path
    ).await.context("Failed to acquire device")?;

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).context("Failed to find surface format")?,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &config);

    debug!("Creating vertex buffer");

    let meshes = vec![
      MeshBuilder::new_poly(
        vec![
          vec3(-1.0, -1.0, 0.0),
          vec3( 1.0, -1.0, 0.0),
          vec3( 1.0,  1.0, 0.0),
          vec3(-1.0,  1.0, 0.0),
        ]
      ).with_color(&RGBA::new(0.5, 0.5, 0.5, 1.0)).build(&device),
      MeshBuilder::new_poly_from_vertices(
        vec![
          Vertex { position: vec3( 0.50,  0.00,  0.00), color: Some(RGBA::new(1.0, 1.0, 1.0, 1.0)) },
          Vertex { position: vec3( 0.40,  0.20,  0.00), color: Some(RGBA::new(1.0, 0.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.20,  0.40,  0.00), color: Some(RGBA::new(1.0, 1.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.00,  0.50,  0.00), color: Some(RGBA::new(0.0, 1.0, 0.0, 1.0)) },
          Vertex { position: vec3(-0.20,  0.40,  0.00), color: Some(RGBA::new(0.0, 1.0, 1.0, 1.0)) },
          Vertex { position: vec3(-0.40,  0.20,  0.00), color: Some(RGBA::new(0.0, 0.0, 1.0, 1.0)) },
          Vertex { position: vec3(-0.50,  0.00,  0.00), color: Some(RGBA::new(1.0, 0.0, 1.0, 1.0)) },
          Vertex { position: vec3(-0.40, -0.20,  0.00), color: Some(RGBA::new(0.0, 0.0, 0.0, 1.0)) },
          Vertex { position: vec3(-0.20, -0.40,  0.00), color: Some(RGBA::new(0.0, 1.0, 1.0, 1.0)) },
          Vertex { position: vec3( 0.00, -0.50,  0.00), color: Some(RGBA::new(0.0, 0.0, 1.0, 1.0)) },
          Vertex { position: vec3( 0.20, -0.40,  0.00), color: Some(RGBA::new(1.0, 0.0, 1.0, 1.0)) },
          Vertex { position: vec3( 0.40, -0.20,  0.00), color: Some(RGBA::new(0.0, 0.0, 0.0, 1.0)) },
        ]
      ).build(&device),
    ];

    debug!("Loading built-in shaders");
    let simple_shader = read_shader!("simple_shader"; &device);

    debug!("Creating render pipeline");
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
        entry_point: "vs_main",
        buffers: &VERTEX_BUFFER_LAYOUTS,
      },
      fragment: Some(wgpu::FragmentState{
        module: &simple_shader,
        entry_point: "fs_main",
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

    debug!("Render system initialized");

    Ok(Self {
      surface,
      device,
      queue,
      config,
      render_pipeline,
      size,
      meshes,
    })
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  pub fn input(&mut self, _event: &WindowEvent) -> bool {
    false
  }

  pub fn start(&mut self) {
    //todo!()
  }

  pub fn tick(&mut self) {
    //todo!()
  }

  pub fn update(&mut self) {
    //todo!()
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output_texture = self.surface.get_current_texture()?;
    let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
      label: Some("Render Encoder")
    });

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 1.00,
              g: 0.00,
              b: 1.00,
              a: 1.00,
            }),
            store: true,
          },
        }],
        depth_stencil_attachment: None,
      });

      render_pass.set_pipeline(&self.render_pipeline);

      for mesh in self.meshes.iter() {
        mesh.draw(&mut render_pass);
      }
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();

    Ok(())
  }
}