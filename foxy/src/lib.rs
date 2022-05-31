//#![feature(backtrace)]
#[macro_use]
pub mod util;
pub mod state;
pub mod mesh;
pub mod vertex;

use anyhow::Context;
use util::{
  log::*,
  error::FoxyError,
};
use state::*;
pub use tokio::{
  self,
  net::TcpListener,
  io::{AsyncReadExt, AsyncWriteExt}
};

use wgpu::SurfaceError;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

pub struct App {
  event_loop: EventLoop<()>,
  window:     Window,
  state:      State,
}

impl App {
  pub async fn new(title: &str, window_size: [u32; 2]) -> Result<Self, FoxyError> {
    logger::init().context("Failed to initialize logger")?;

    debug!("Initializing app");

    let event_loop = winit::event_loop::EventLoop::new();

    let (logical_size, _physical_size) = {
      use winit::dpi::{LogicalSize, PhysicalSize};

      let dpi = event_loop.primary_monitor()
        .context("Failed to find primary monitor.")?
        .scale_factor();
      let logical: LogicalSize<u32> = window_size.into();
      let physical: PhysicalSize<u32> = logical.to_physical(dpi);

      (logical, physical)
    };

    debug!("Building window");

    let window = WindowBuilder::new()
      .with_title(title)
      .with_inner_size(logical_size)
      .with_visible(false)
      .build(&event_loop)
      .context("Failed to build window")?;

    let state = State::new(&window).await?;

    debug!("App initialized");

    Ok(App {
      event_loop,
      window,
      state,
    })
  }

  pub async fn run(mut self) {
    debug!("Starting app");
    self.state.start();

    self.event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;
      match event {
        // Handle window events only if the input states are satisfied first
        Event::WindowEvent { event, .. } if !self.state.input(&event) => match event {
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
            debug!("Stopping app");
          },
          WindowEvent::Resized(dims) => {
            self.state.resize(dims);
          }
          WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            self.state.resize(*new_inner_size);
          }
          _ => (),
        },
        Event::MainEventsCleared => {
          self.state.tick();
          self.state.update();
          match self.state.render() {
            // Reconfigure the surface if it's lost or outdated
            Err(SurfaceError::Lost | SurfaceError::Outdated) =>
              self.state.resize(self.state.size),
            // The system is out of memory, we should probably quit
            Err(SurfaceError::OutOfMemory) => {
              *control_flow = ControlFlow::Exit;
              error!("System out of memory!");
            },
            Err(SurfaceError::Timeout) => warn!("Surface timeout"),
            Ok(_) => {
              // Upon successful first rendering, reveal the window
              self.window.set_visible(true);
            }
          }
        },
        _ => (),
      }
    });
  }
}
