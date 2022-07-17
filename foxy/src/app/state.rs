use std::time::Duration;
use crate::{
  util::{
    log::*,
    error::FoxyError,
  },
  graphics::{
    mesh::MeshBuilder,
    renderer::Renderer,
    vertex::Vertex
  }
};
use anyhow::Context;
use wgpu::Instance;
use winit::{
  event::*,
  window::Window,
};
use glam::*;
use rgb::RGBA;
use foxy_ecs::{
  entity::Entity,
  component::Name
};

pub struct State {
  pub surface: wgpu::Surface,
  pub renderer: Renderer,
  pub size: winit::dpi::PhysicalSize<u32>
}

impl State {
  pub async fn new(window: &Window) -> Result<Self, FoxyError> {
    trace!("> Intializing render system...");
    let size = window.inner_size();
    let instance = Instance::new(wgpu::Backends::VULKAN);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).await.context("> Failed to acquire adapter")?;
    info!("Supported features: {:?}", adapter.features());
    let renderer = Renderer::new(&surface, &adapter, size).await?;
    trace!("> Render system initialized!");

    Ok(Self {
      surface,
      renderer,
      size,
    })
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.renderer.config.width = new_size.width;
      self.renderer.config.height = new_size.height;
      self.surface.configure(&self.renderer.device, &self.renderer.config);
    }
  }

  pub fn input(&mut self, _event: &WindowEvent) -> bool {
    false
  }

  pub fn start(&mut self) {
    //todo!()
    let e0 = Entity::new().add_component(Name("e0".into()));
    let e1 = Entity::new().remove_component::<Name>();
    let e2 = Entity::new();
  }

  pub fn tick(&mut self, time_delta: Duration) {
    //todo!()
  }

  pub fn update(&mut self, time_delta: Duration) {
    //todo!()
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output_texture = self.surface.get_current_texture()?;
    let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
      label: Some("Render Encoder")
    });

    let meshes = vec![
      MeshBuilder::new_poly(
        vec![
          vec3(-1.0, -1.0, 0.0),
          vec3( 1.0, -1.0, 0.0),
          vec3( 1.0,  1.0, 0.0),
          vec3(-1.0,  1.0, 0.0),
        ]
      ).with_color(&RGBA::new(0.5, 0.5, 0.5, 1.0)).build(&self.renderer),
      MeshBuilder::new_poly_from_vertices(
        vec![
          Vertex { position: vec3(-0.50, -0.50,  0.00), color: Some(RGBA::new(1.0, 0.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.50, -0.50,  0.00), color: Some(RGBA::new(0.0, 1.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.50,  0.50,  0.00), color: Some(RGBA::new(0.0, 0.0, 1.0, 1.0)) },
          Vertex { position: vec3(-0.50,  0.50,  0.00), color: Some(RGBA::new(1.0, 1.0, 0.0, 1.0)) },
        ]
      ).build(&self.renderer),
      MeshBuilder::new_poly_from_vertices(
        vec![
          Vertex { position: vec3(-0.50, -0.50,  0.00), color: Some(RGBA::new(1.0, 0.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.50, -0.50,  0.00), color: Some(RGBA::new(0.0, 1.0, 0.0, 1.0)) },
          Vertex { position: vec3( 0.00,  0.50,  0.00), color: Some(RGBA::new(0.0, 0.0, 1.0, 1.0)) },
        ]
      ).build(&self.renderer),
    ];

    {
      let per_frame_data = self.renderer.prepare(meshes);

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

      self.renderer.render(&mut render_pass, &per_frame_data);
    }

    // submit will accept anything that implements IntoIter
    self.renderer.queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();

    Ok(())
  }
}