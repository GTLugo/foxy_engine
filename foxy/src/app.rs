use std::time::*;

use crate::{
  util::{
    error::FoxyError,
    log::logger
  },
};
use anyhow::Context;

use tracing::{error, trace, warn};
use wgpu::SurfaceError;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use state::State;

pub mod state;

pub struct App {
  event_loop: EventLoop<()>,
  window: Window,
  state: State,
  time_instant_previous: Instant,
  time_lag: Duration,
  time_ms_per_update: Duration,
  time_delta: Duration,
}

impl App {
  pub async fn new(title: &str, window_size: [u32; 2]) -> Result<Self, FoxyError> {
    logger::init().context("Failed to initialize logger")?;

    trace!("Initializing app...");

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

    trace!("Building window...");

    let window = WindowBuilder::new()
      .with_title(title)
      .with_inner_size(logical_size)
      .with_visible(false)
      .build(&event_loop)
      .context("Failed to build window")?;

    let state = State::new(&window).await?;

    trace!("App initialized!");

    Ok(App {
      event_loop,
      window,
      state,
      time_instant_previous: Instant::now(),
      time_lag: Duration::default(),
      time_ms_per_update: Duration::from_secs_f64(1.0 / 128.0),
      time_delta: Duration::default(),
    })
  }

  pub async fn run(mut self) {
    trace!("Starting app");
    self.state.start();

    self.event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;
      match event {
        // Handle window events only if the input states are satisfied first
        Event::WindowEvent { event, .. } if !self.state.input(&event) => match event {
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
            trace!("Stopping app");
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
          self.time_delta = self.time_instant_previous.elapsed();
          self.time_instant_previous = Instant::now();
          self.time_lag += self.time_delta;

          self.state.update(self.time_delta);
          while self.time_lag >= self.time_ms_per_update {
            self.state.tick(self.time_delta);
            self.time_lag -= self.time_ms_per_update;
          }

          match self.state.render() {
            // Reconfigure the surface if it's lost or outdated
            Err(SurfaceError::Lost | SurfaceError::Outdated) =>
              self.state.resize(self.state.size),
            // The system is out of memory, we should probably quit
            Err(SurfaceError::OutOfMemory) => {
              *control_flow = ControlFlow::Exit;
              error!("System out of memory!");
            },
            Err(SurfaceError::Timeout) => warn!("Surface timeout!"),
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

  // pub fn add_global_data<C: Component>(&mut self, data: C) {
  //
  // }
}
