use std::collections::HashMap;

pub const START_STATE: usize = 0;
pub const EPSILON: char = '\0';

pub struct State {
  pub id: usize,
  pub transitions: HashMap<char, Vec<usize>>,
  pub success: bool,
}

impl State {
  pub fn new(id: usize) -> Self {
    return State {
      id,
      transitions: HashMap::new(),
      success: false,
    };
  }

  pub fn create_transition(&mut self, dst: usize, c: char) {
    match self.transitions.get_mut(&c) {
      Some(s) => { s.push(dst); },
      None => { self.transitions.insert(c, vec![dst]); },
    }
  }
}

pub struct NFA {
  pub states: Vec<State>,
}

impl NFA {
  pub fn new() -> Self {
    return NFA {
      states: vec![],
    };
  }
}
