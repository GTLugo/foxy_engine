#[allow(unused_imports)]
#[macro_use]
extern crate glium;
mod log;

pub mod app {
  #[allow(unused_imports)]
  use crate::{fox_debug, fox_error, fox_trace, log::logging::*};
  
  //#[allow(unused_imports)]
  use glium::{Display, Frame, Surface, glutin::{ContextBuilder, dpi::{PhysicalPosition, PhysicalSize}, event::*, event_loop::*, window::WindowBuilder}};
    
  #[cfg(target_os = "windows")]
  use glium::glutin::platform::windows::EventLoopExtWindows;
  #[cfg(target_os = "linux")]
  use glium::glutin::platform::unix::EventLoopExtUnix;
    

  pub struct AppInfo {
    pub title: &'static str,
    pub width: u32,
    pub height: u32
  }
  struct AppState {
    pub control_flow: ControlFlow,
    pub mouse_location: PhysicalPosition<f64>
  }

  pub struct App {
    display: Option<Box<Display>>,
    info: AppInfo,
    state: AppState
  }

  impl App {
    pub fn new(info: AppInfo) -> Self {
      match setup_logging() {
        Ok(_) => { fox_trace!("ENGINE", "logger initialized!") }
        Err(_) => { fox_error!("ENGINE", "failed to initialize logger!") }
      };
      let state = AppState {
        control_flow: ControlFlow::Poll,
        mouse_location: PhysicalPosition{x: 0.0, y: 0.0}
      };

      Self {
        display: None,
        info,
        state
      }
    }

    pub fn run(mut self) {
      let event_loop = Box::new(EventLoop::new_any_thread());
      let wb = WindowBuilder::new()
        .with_title(self.info.title)
        .with_inner_size(PhysicalSize{width: self.info.width, height: self.info.height})
        .with_decorations(false);
      let cb = ContextBuilder::new();
      self.display = Some(Box::new(Display::new(wb, cb, &event_loop).unwrap_or_log()));

      event_loop.run(move |event, _, control_flow| {
        self.state.control_flow = *control_flow;
        self.update(&event);
        *control_flow = self.state.control_flow;
      });
    }

    fn update(&mut self, e: &Event<()>) {
      let frame = self.display.as_ref().unwrap_or_log().draw();
      Self::render(frame);
      
      self.state.control_flow = ControlFlow::Poll;
      match e {
        Event::WindowEvent { event, window_id} => 
          match event {
            WindowEvent::CloseRequested => {
              if *window_id == self.display.as_ref().unwrap_or_log().gl_window().window().id() {
                self.state.control_flow = ControlFlow::Exit;
              }
            },
            WindowEvent::CursorMoved {device_id: _, position, .. } => {
              self.state.mouse_location = *position;
            },
            WindowEvent::MouseInput { device_id: _, state, button, .. } => {
              match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => {
                  

                  fox_debug!("FOXY", "mouse left pressed");
                  self.display.as_ref().unwrap_or_log().gl_window().window().drag_window().unwrap_or_log();
                },
                _ => (),
              }
            },
            WindowEvent::KeyboardInput { input, is_synthetic: _, .. } => {
              match input.virtual_keycode {
                Some(keycode) => {
                  match keycode {
                    VirtualKeyCode::Escape => {
                      self.state.control_flow = ControlFlow::Exit;
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
