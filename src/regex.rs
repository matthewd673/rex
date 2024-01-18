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

  fn interpret(&mut self, tree: &TreeNode, start: usize) -> (bool, usize) {
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

      if start < end {
        return (success, end);
      }
      else {
        return (success, end + 1);
      }
    }
    // if attempt failed, end = start and match_all will enter infinite loop
    else {
      return (success, end + 1);
    }
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
      if r.includes_char(self.chars[i]) {
        return (true, i + 1);
      }
    }

    // character not included in range
    return (false, i);
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

  pub fn match_first(&self, s: String) -> Option<MatchData> {
    let mut m = RegExEnv::new(s);
    let mut start = 0;
    while start < m.string.len() {
      let (success, end) = m.interpret(&self.tree, start);
      start = end;

      // return as soon as a match is found
      if success {
        return m.matches.pop(); // if its mysteriously missing this still works
      }
    }

    // nothing was found
    return None;
  }

  pub fn match_all(&self, s: String) -> Vec<MatchData> {
    let mut m = RegExEnv::new(s);
    let mut start = 0;
    while start <= m.string.len() {
      let (_, end) = m.interpret(&self.tree, start);
      start = end;
    }

    return m.matches;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn match_character() {
    let r = RegEx::new("a");
    let m = r.match_first(String::from("a"));

    assert!(m.is_some());

    let mu = m.unwrap();
    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 1);
  }

  #[test]
  fn miss_character() {
    let r = RegEx::new("b");
    let m = r.match_first(String::from("a"));
    assert!(m.is_none());
  }

  #[test]
  fn match_all_characters() {
    let r = RegEx::new("a");
    let m = r.match_all(String::from("aaaaa"));
    assert_eq!(m.len(), 5);

    for i in 0..m.len() {
      assert_eq!(m[i].start, i);
      assert_eq!(m[i].end, i + 1);
    }
  }

  #[test]
  fn miss_all_characters() {
    let r = RegEx::new("b");
    let m = r.match_all(String::from("aaaaa"));
    assert_eq!(m.len(), 0);
  }

  #[test]
  fn match_sequence() {
    let r = RegEx::new("abc");
    let m = r.match_first(String::from("abc"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 3);
  }

  #[test]
  fn miss_sequence() {
    let r = RegEx::new("abc");
    let m = r.match_first(String::from("axc"));
    assert!(m.is_none());
  }

  #[test]
  fn match_all_sequences() {
    let r = RegEx::new("abc");
    let m = r.match_all(String::from("abcabcabc"));

    assert_eq!(m.len(), 3);

    for i in 0..m.len() {
      assert_eq!(m[i].start, i * 3);
      assert_eq!(m[i].end, i * 3 + 3);
    }
  }

  #[test]
  fn miss_all_sequences() {
    let r = RegEx::new("abc");
    let m = r.match_all(String::from("axcxbcabx"));
    assert_eq!(m.len(), 0);
  }

  #[test]
  fn match_sequence_union() {
    let r = RegEx::new("abc|xyz");
    let m = r.match_first(String::from("ab_xyzabc"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 3);
    assert_eq!(mu.end, 6);
  }

  #[test]
  fn miss_sequence_union() {
    let r = RegEx::new("abc|xyz");
    let m = r.match_first(String::from("aaaaaa"));
    assert!(m.is_none());
  }

  #[test]
  fn match_all_sequence_union() {
    let r = RegEx::new("abc|xyz");
    let m = r.match_all(String::from("xyzabcddd"));

    assert_eq!(m.len(), 2);
    assert_eq!(m[0].start, 0);
    assert_eq!(m[0].end, 3);
    assert_eq!(m[1].start, 3);
    assert_eq!(m[1].end, 6);
  }

  #[test]
  fn miss_all_sequence_union() {
    let r = RegEx::new("abc|xyz");
    let m = r.match_all(String::from("defdefdef"));
    assert_eq!(m.len(), 0);
  }

  #[test]
  fn match_character_kleene_exists() {
    let r = RegEx::new("a*");
    let m = r.match_first(String::from("aaaa"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 4);
  }

  // NOTE: match_character_kleene_exists cannot miss

  #[test]
  fn match_character_kleene_doesnt_exist() {
    let r = RegEx::new("a*");
    let m = r.match_first(String::from("bbbb"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 0);
  }

  // NOTE: match_character_kleene_doesnt_exist cannot miss

  #[test]
  fn match_all_character_kleene_exists() {
    let r = RegEx::new("a*");
    let m = r.match_all(String::from("aaaa"));

    assert_eq!(m.len(), 2);

    assert_eq!(m[0].start, 0);
    assert_eq!(m[0].end, 4);
    assert_eq!(m[1].start, 4);
    assert_eq!(m[1].end, 4);
  }

  // NOTE: match_all_character_kleene_exists cannot miss

  #[test]
  fn match_all_character_kleene_doesnt_exist() {
    let r = RegEx::new("a*");
    let m = r.match_all(String::from("bbbb"));

    assert_eq!(m.len(), 5);

    for i in 0..m.len() {
      assert_eq!(m[i].start, i);
      assert_eq!(m[i].end, i);
    }
  }

  // NOTE: match_all_character_kleene_doesnt_exist cannot miss

  #[test]
  fn match_sequence_and_kleene() {
    let r = RegEx::new("abc*");
    let m = r.match_first(String::from("abccc"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 5);
  }

  #[test]
  fn miss_sequence_and_kleene() {
    let r = RegEx::new("abc*");
    let m = r.match_first(String::from("def"));
    assert!(m.is_none());
  }

  #[test]
  fn match_all_sequence_and_kleene() {
    let r = RegEx::new("abc*");
    let m = r.match_all(String::from("abcccababc"));

    assert_eq!(m.len(), 3);

    assert_eq!(m[0].start, 0);
    assert_eq!(m[0].end, 5);
    assert_eq!(m[1].start, 5);
    assert_eq!(m[1].end, 7);
    assert_eq!(m[2].start, 7);
    assert_eq!(m[2].end, 10);
  }

  #[test]
  fn miss_all_sequence_and_kleene() {
    let r = RegEx::new("abc*");
    let m = r.match_all(String::from("ccca"));
    assert_eq!(m.len(), 0);
  }

  #[test]
  fn match_kleene_within_union() {
    let r = RegEx::new("abc|a*");
    let m = r.match_first(String::from("aaaaabc"));
    assert!(m.is_some());

    let mu = m.unwrap();

    assert_eq!(mu.start, 0);
    assert_eq!(mu.end, 5);
  }

  // NOTE: match_kleene_within_union cannot miss

  #[test]
  fn match_all_kleene_within_union() {
    let r = RegEx::new("abc|a*");
    let m = r.match_all(String::from("aaabcabc"));

    assert_eq!(m.len(), 5);

    assert_eq!(m[0].start, 0);
    assert_eq!(m[0].end, 3);
    assert_eq!(m[1].start, 3);
    assert_eq!(m[1].end, 3);
    assert_eq!(m[2].start, 4);
    assert_eq!(m[2].end, 4);
    assert_eq!(m[3].start, 5);
    assert_eq!(m[3].end, 8);
    assert_eq!(m[4].start, 8);
    assert_eq!(m[4].end, 8);
  }

  // NOTE: match_all_kleene_within_union cannot miss
}
