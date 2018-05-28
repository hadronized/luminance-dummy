extern crate luminance;

use std::cell::RefCell;
use std::rc::Rc;
use luminance::context::GraphicsContext;
use luminance::state::{GraphicsState, StateQueryError};

pub struct DummyContext {
  graphics_state: Rc<RefCell<GraphicsState>>
}

impl DummyContext {
  pub fn new() -> Option<Self> {
    GraphicsState::new().map(|state| {
      println!("creating context…");
      DummyContext { graphics_state: Rc::new(RefCell::new(state))  }
    }).ok()
  }
}

unsafe impl GraphicsContext for DummyContext {
  fn state(&self) -> &Rc<RefCell<GraphicsState>> {
    &self.graphics_state
  }

  fn swap_buffers(&mut self) {
    println!("swapping buffers!");
  }
}
