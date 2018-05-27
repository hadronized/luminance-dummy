extern crate luminance;

use std::cell::RefCell;
use std::rc::Rc;
use luminance::context::{GraphicsContext, WithGraphicsState, thread_acquire_context};
use luminance::state::{GraphicsState, StateQueryError};

pub struct DummyContext {
  graphics_state: Rc<RefCell<GraphicsState>>
}

impl DummyContext {
  pub fn new() -> Option<Self> {
    thread_acquire_context(DummyContextBuilder)
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

struct DummyContextBuilder;

impl WithGraphicsState for DummyContextBuilder {
  type Output = DummyContext;

  fn call_once<F>(self, gfx_state: F) -> Self::Output where F: FnOnce() -> Result<GraphicsState, StateQueryError> {
    println!("creating context…");

    let state = gfx_state().expect("graphics state");
    DummyContext { graphics_state: Rc::new(RefCell::new(state))  }
  }
}
