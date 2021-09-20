#[allow(unused_imports)]
#[macro_use]
extern crate glium;

pub mod foxy {
  #[allow(unused_imports)]
  use glium::{
      Display, Frame, Surface,
      glutin::{
        self, 
        event, event::*, 
        event_loop, event_loop::*,
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
      let event_loop = Box::new(EventLoop::new_any_thread());
      let wb = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize{width, height})
        .with_decorations(false);
      let cb = ContextBuilder::new();
      let display = Box::new(Display::new(wb, cb, &event_loop).unwrap());
      Self {
        event_loop,
        display
      }
    }

    pub fn run(self) {
      self.event_loop.run(move |e, _, control_flow| {
        let mut control_data = EventLoopData {
            event: e,
            control_flow,
        };
        
        Self::update(&mut control_data, &self.display);
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
            WindowEvent::MouseInput { device_id, state, button, .. } => {
              match button {
                  MouseButton::Left => {
                    display.gl_window().window().drag_window().unwrap();
                  },
                  _ => (),
              }
            },
            _ => (),
          },
        _ => (),
      }
    }
        
    fn render(mut frame: Frame) {
      frame.clear_color_srgb(0.10, 0.13, 0.16, 1.0);
      frame.finish().unwrap();
    }
  }
}
