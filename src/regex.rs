use std::collections::HashMap;
use std::str::Chars;

const START_STATE: usize = 0;
const EPSILON: char = '\0';

struct State {
  id: usize,
  transitions: HashMap<char, Vec<usize>>,
  success: bool,
}

impl State {
  fn new(id: usize) -> Self {
    return State {
      id,
      transitions: HashMap::new(),
      success: false,
    };
  }

  fn create_transition(&mut self, dst: usize, c: char) {
    match self.transitions.get_mut(&c) {
      Some(s) => { s.push(dst); },
      None => { self.transitions.insert(c, vec![dst]); },
    }
  }
}

struct NFA {
  states: Vec<State>,
}

impl NFA {
  fn new() -> Self {
    return NFA {
      states: vec![],
    };
  }

  fn count(&mut self) -> usize {
    return self.states.len();
  }

  // fn create_state(&mut self) -> usize {
    // let new = State::new(self.count());
    // self.states.push(new);
    // return self.count() - 1;
  // }

  // fn create_transition(&mut self, src: usize, dst: usize, c: char) {
    // self.states[src].create_transition(dst, c);
  // }

  // fn mark_success(&mut self, state: usize, success: bool) {
    // self.states[state].success = success;
  // }
}

struct ParseResult {
  head: usize,
  tail: usize,
  success: bool,
}

fn compile(expr: &str) -> NFA {
  let escape = false;

  let mut nfa = NFA::new();

  // TODO: temp
  let mut a = State::new(0);
  let mut b = State::new(1);

  a.create_transition(b.id, 'a');
  b.create_transition(a.id, 'b');
  b.create_transition(a.id, EPSILON);

  a.success = true;

  nfa.states.push(a);
  nfa.states.push(b);

  return nfa;
}

pub struct RegEx {
  nfa: NFA,
}

impl RegEx {
  pub fn new(expr: &str) -> Self {
    return RegEx {
      nfa: compile(expr),
    };
  }

  pub fn eval(&self, s: &str) -> bool {
    let mut chars = s.chars();
    let mut nexts: Vec<&usize> = vec![&START_STATE];
    let mut has_success = false;

    // execute on the string
    loop {
      let states = nexts.clone();
      nexts.clear();

      // println!("states:");
      // for s in &states {
        // println!("  {}", s);
      // }

      let c = chars.next();
      match c {
        // read the next character
        Some(c) => {
          // println!("looking at {}", c);
          // try to transition from every state
          for s in &states {
            // try to transition
            match self.nfa.states[**s].transitions.get(&c) {
              // transition
              Some(dsts) => {
                for d in dsts {
                  nexts.push(d);
                  if self.nfa.states[*d].success {
                    has_success = true;
                  }
                }
              }
              // no transition
              None => { },
            }
          }
        },
        // no more characters, break
        None => { break; }
      }

      // also add epsilon states
      // pretty much copy-paste from above
      for s in states {
        match self.nfa.states[*s].transitions.get(&EPSILON) {
          // transition
          Some(dsts) => {
            for d in dsts {
              nexts.push(d);
              if self.nfa.states[*d].success {
                has_success = true;
              }
            }
          },
          // no transition
          None => { },
        }
      }

      // println!("nexts:");
      // for n in &nexts {
        // println!("  {}", n);
      // }

      // stop if no more states to hit
      if nexts.len() == 0 {
        break;
      }

      // println!("----");
    }

    // return result
    return has_success;
  }
}
