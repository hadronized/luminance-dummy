extern crate luminance;
extern crate luminance_dummy;
extern crate luminance_glfw;

use luminance::context::GraphicsContext;
use luminance_glfw::events::{Action, Key, WindowEvent};
use luminance_glfw::window::{self, WindowDim, WindowOpt};
use std::sync::mpsc;
use std::thread;

enum Cmd {
  Quit
}

fn main() {
  let (mut surface, mut events) = window::new(WindowDim::Windowed(800, 600), "test", WindowOpt::default()).expect("window::new");

  // channel between event loop and render loop
  let (cmd_sx, cmd_rx) = mpsc::channel();
  // event loop
  thread::spawn(move || {
    loop {
      for event in events.wait_iter() {
        match event {
          WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
            let _ = cmd_sx.send(Cmd::Quit);
          }
          _ => ()
        }
      }
    }
  });

  // render and main loop
  'app: loop {
    // exit if the event loop asks to
    if let Ok(cmd) = cmd_rx.try_recv() {
      match cmd {
        Cmd::Quit => break 'app
      }
    }

    println!("render!");
    surface.swap_buffers();
  }
}
