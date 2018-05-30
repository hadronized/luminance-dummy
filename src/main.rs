extern crate luminance;
extern crate luminance_dummy;
extern crate luminance_glfw;

use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(800, 600), "test", WindowOpt::default()).expect("window::new");
  let mut screen_color = [0., 0., 0., 1.];

  // render and main loop
  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        WindowEvent::Key(Key::R, _, Action::Release, _) => screen_color = [1., 0., 0., 1.],
        WindowEvent::Key(Key::G, _, Action::Release, _) => screen_color = [0., 1., 0., 1.],
        WindowEvent::Key(Key::B, _, Action::Release, _) => screen_color = [0., 0., 1., 1.],
        _ => ()
      }
    }

    let builder = surface.pipeline_builder();
    let screen = Framebuffer::default(surface.size());
    builder.pipeline(&screen, screen_color, |_, _| {
    });

    surface.swap_buffers();
  }
}
