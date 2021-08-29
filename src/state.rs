use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct State<T> {
  pub state: T,
  pub prev_state: Option<T>,
}

impl<T: fmt::Debug> fmt::Display for State<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} -> {:?}", self.prev_state, self.state)
  }
}

// Wrapper for module state to allow performing any state diffs
impl<T: Clone> State<T> {
  pub fn from(s: T) -> Self {
    State {
      prev_state: None,
      state: s,
    }
  }

  pub fn get(&self) -> &T {
    &self.state
  }

  pub fn set(&mut self, val: T) {
    self.update_prev_state();
    self.state = val;
  }

  pub fn update(&mut self, updater: impl Fn(&mut T)) {
    self.update_prev_state();
    updater(&mut self.state);
  }

  // Update previous state to be current state
  pub fn update_prev_state(&mut self) {
    self.prev_state = Some(self.state.clone());
  }

  // NOTE: alias for update_prev_state. To be used at the end of frame
  pub fn processed(&mut self) {
    self.update_prev_state();
  }
}

//
// Check if state has changed
//
// Usage -
//    has_changed(self, [property1 property2], {
//      do_stuff_on_change();
//    });
//
#[macro_export]
macro_rules! has_changed {
  ($expr:expr, [$($prop:ident)*], $body:block) => {
    if $(
      ($expr.$prop.prev_state.is_some() && $expr.$prop.state != $expr.$prop.prev_state.unwrap())
    )||+
      $body
  };
}

