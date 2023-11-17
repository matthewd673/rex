use crate::nfa::NFA;
use crate::nfa::State;
use crate::nfa::START_STATE;
use crate::nfa::EPSILON;

fn compile(expr: &str) -> NFA {
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

      let next_char = chars.next();
      match next_char {
        // read the next character
        Some(c) => {
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

      // stop if no more states to hit
      if nexts.len() == 0 {
        break;
      }
    }

    // return result
    return has_success;
  }
}
