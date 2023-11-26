use crate::parser::Parser;
use crate::parser::TreeNode;
use crate::parser::NodeType;
use std::collections::VecDeque;

pub struct MatchGroup {
  pub start: usize,
  pub end: usize,
  pub string: String,
}

pub struct MatchData {
  pub start: usize,
  pub end: usize,
  pub groups: VecDeque<MatchGroup>,
}

impl MatchData {
  fn new(start: usize, end: usize) -> Self {
    return MatchData {
      start,
      end,
      groups: VecDeque::new(),
    };
  }
}

pub struct RegExEnv {
  chars: Vec<char>,
  pub string: String,
  pub matches: Vec<MatchData>,
}

impl RegExEnv {
  fn new(s: String) -> Self {
    return RegExEnv {
      chars: (&s).chars().collect(),
      string: s,
      // the below should be set before the match is returned
      matches: vec![],
    };
  }

  fn interpret(&mut self, tree: &TreeNode, start: usize) -> usize {
    let mut new_match = MatchData::new(start, 0);
    let (success, end) = self.interpret_node(tree, start, &mut new_match);

    // if matched, create new MatchData
    if success {
      new_match.end = end;
      new_match.groups.push_front(MatchGroup {
          start,
          end,
          string: String::from(&self.string[start..end]),
      });
      self.matches.push(new_match);
    }

    return end;
  }

  fn print_chars(&self, i: usize) -> String {
    let mut s = String::new();
    let mut ci = i;
    while ci < self.chars.len() {
      s.push(self.chars[ci]);
      ci += 1;
    }

    return s;
  }

  fn interpret_node(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    match &node.n_type {
      NodeType::Word => self.interpret_word(node, i, m),
      NodeType::Union => self.interpret_union(node, i, m),
      NodeType::Star => self.interpret_star(node, i, m),
      NodeType::Group => self.interpret_group(node, i, m),
      NodeType::MatchGroup => self.interpret_match_group(node, i, m),
      NodeType::Charset => self.interpret_charset(node, i, m),
      _ => {
        println!("runtime error: unknown node type {:?}", node.n_type);
        return (false, i);
      },
    }
  }

  fn interpret_word(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    let mut w_i = i;
    // try to match every char in word
    for c in &node.image {
      // reached the end of the string, can't possibly match
      if w_i >= self.chars.len() {
        return (false, w_i);
      }

      // break if mismatch
      if &self.chars[w_i] != c {
        return (false, w_i);
      }
      w_i += 1;
    }

    return (true, w_i); // nothing bad happened
  }

  fn interpret_union(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    let mut success = false;
    let mut best_i = i;
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, i, m);
      success = success | n_s;
      if n_s && n_i > best_i {
        best_i = n_i;
      }
    }

    return (success, best_i);
  }

  fn interpret_star(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    // there will only ever be one child
    let n = &node.children[0];
    let mut best_i = i;

    // interpret that child as long as possible
    let mut loop_ct = 0;
    loop {
      let (n_s, n_i) = self.interpret_node(n, best_i, m);
      if !n_s {
        break;
      }

      best_i = n_i;
      loop_ct += 1;

      // stop if maximum has been exceeded
      // if maximum is zero then it will never be exceeded (a.k.a: 0 = inf)
      if loop_ct == node.repeats.max {
        break;
      }
    }

    // only a success if minimum count was reached
    return (loop_ct >= node.repeats.min, best_i);
  }

  fn interpret_group(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    let mut last_i = i;

    // just interpret all children
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, last_i, m);
      if !n_s {
        return (false, n_i);
      }

      last_i = n_i;
    }

    return (true, last_i);
  }

  fn interpret_match_group(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    // println!("interpret_match_group @ {}", i);
    // just interpret a group like normal
    let (g_s, g_i) = self.interpret_group(node, i, m);

    if g_s {
      m.groups.push_back(MatchGroup {
          start: i,
          end: g_i,
          string: String::from(&self.string[i..g_i]),
        });
    }

    return (g_s, g_i);
  }

  fn interpret_charset(&self, node: &TreeNode, i: usize, m: &mut MatchData)
    -> (bool, usize) {
    // skip if out of bounds
    if i >= self.chars.len() {
      return (false, i);
    }

    // try to find any match in char set
    for r in &node.ranges {
      if !node.negated &&
         &self.chars[i] >= &r.min && &self.chars[i] <= &r.max {
        return (true, i + 1);
      }
      else if node.negated &&
              (&self.chars[i] >= &r.min && &self.chars[i] <= &r.max) {
        return (false, i);
      }
    }

    // no match found for positive match (bad)
    if !node.negated {
      return (false, i);
    }
    // no match found for negative match (good)
    else {
      return (true, i + 1);
    }
  }
}

pub struct RegEx {
  pub expr: String,
  tree: TreeNode,
}

impl RegEx {
  pub fn new(expr: &str) -> Self {
    let expr = String::from(expr);
    let mut parser = Parser::new(&expr);
    return RegEx { expr, tree: parser.parse() };
  }

  pub fn match_all(&self, s: String) -> Vec<MatchData> {
    let mut m = RegExEnv::new(s);
    let mut start = 0;
    while start < m.string.len() {
      start = m.interpret(&self.tree, start) + 1;
    }

    return m.matches;
  }
}
