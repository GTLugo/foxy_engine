#[allow(unused_imports)]
#[macro_use]
extern crate glium;
mod log;

pub mod foxy {
  #[allow(unused_imports)]
  use crate::{fox_debug, fox_error, fox_trace, log::foxy::*};
  
  //#[allow(unused_imports)]
  use glium::{
      Display, Frame, Surface,
      glutin::{
        event::*, 
        event_loop::*,
        window::WindowBuilder,
        ContextBuilder,
        dpi::PhysicalSize
      }
  };
    
  #[cfg(target_os = "windows")]
  use glium::glutin::platform::windows::EventLoopExtWindows;
  #[cfg(target_os = "linux")]
  use glium::glutin::platform::unix::EventLoopExtUnix;
    
  struct EventLoopData<'a> {
    pub event: Event<'a, ()>, 
    pub control_flow: &'a mut ControlFlow
  }

  pub struct App {
    event_loop: Box<EventLoop<()>>,
    display: Box<Display>
  }

  impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
      match setup_logging() {
        Ok(_) => { fox_trace!("ENGINE", "logger initialized!") }
        Err(_) => { fox_error!("ENGINE", "failed to initialize logger!") }
      };

      let event_loop = Box::new(EventLoop::new_any_thread());
      let wb = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize{width, height})
        .with_decorations(false);
      let cb = ContextBuilder::new();
      let display = Box::new(Display::new(wb, cb, &event_loop).unwrap_or_log());
      Self {
        event_loop,
        display
      }
    }

    pub fn run(self) {
      self.event_loop.run(move |e, _, control_flow| {
        let mut event_loop_data = EventLoopData {
            event: e,
            control_flow
        };
        
        Self::update(&mut event_loop_data, &self.display);
      });
    }

    fn update(data: &mut EventLoopData, display: &Display) {
      let frame = display.draw();
      Self::render(frame);
      
      *data.control_flow = ControlFlow::Poll;
      match &data.event {
        Event::WindowEvent { event, window_id} => 
          match event {
            WindowEvent::CloseRequested => {
              if *window_id == display.gl_window().window().id() {
                *data.control_flow = ControlFlow::Exit;
              }
            },
            WindowEvent::MouseInput { device_id: _, state, button, .. } => {
              match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => {
                  fox_debug!("FOXY", "mouse left pressed");
                  display.gl_window().window().drag_window().unwrap();
                },
                _ => (),
              }
            },
            WindowEvent::KeyboardInput { input, is_synthetic: _, .. } => {
              match input.virtual_keycode {
                Some(keycode) => {
                  match keycode {
                    VirtualKeyCode::Escape => {
                      *data.control_flow = ControlFlow::Exit;
                    },
                    _ => (),
                  }
                },
                None => (),
              }
            }
            _ => (),
          },
        _ => (),
      }
    }
        
    fn render(mut frame: Frame) {
      frame.clear_color_srgb(0.10, 0.13, 0.16, 1.0);
      frame.finish().unwrap_or_log();
    }
  }
}
