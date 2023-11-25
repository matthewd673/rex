use crate::parser::Parser;
use crate::parser::TreeNode;
use crate::parser::NodeType;

pub struct RegExMatch {
  chars: Vec<char>,
  pub success: bool,
}

impl RegExMatch {
  fn new(str: String) -> Self {
    return RegExMatch {
      chars: str.chars().collect(),
      success: false, // should be set later
    };
  }

  fn interpret(&mut self, tree: &TreeNode) {
    let (success, _) = self.interpret_node(tree, 0);
    self.success = success;
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

  fn interpret_node(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    match &node.n_type {
      NodeType::Word => self.interpret_word(node, i),
      NodeType::Union => self.interpret_union(node, i),
      NodeType::Star => self.interpret_star(node, i),
      NodeType::Group => self.interpret_group(node, i),
      NodeType::MatchGroup => self.interpret_match_group(node, i),
      _ => {
        println!("runtime error: unknown node type {:?}", node.n_type);
        return (false, i);
      },
    }
  }

  fn interpret_word(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // println!("word (\"{}\") @ {}", node.image_to_str(), self.print_chars(i));
    let mut w_i = i;
    // try to match every char in word
    for c in &node.image {
      // reached the end of the string, can't possibly match
      if w_i >= self.chars.len() {
        // println!("word -> false (over len)");
        return (false, w_i);
      }

      // break if mismatch
      if &self.chars[w_i] != c {
        // println!("word -> false (mismatch)");
        return (false, w_i);
      }
      w_i += 1;
    }

    // println!("word -> true {}", w_i);
    return (true, w_i); // nothing bad happened
  }

  fn interpret_union(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // println!("union @ {}", self.print_chars(i));
    let mut success = false;
    let mut best_i = i;
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, i);
      success = success | n_s;
      if n_s && n_i > best_i {
        best_i = n_i;
      }
    }

    // println!("union -> {}", success);
    return (success, best_i);
  }

  fn interpret_star(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // println!("star @ {}", self.print_chars(i));
    // there will only ever be one child
    let n = &node.children[0];
    let mut best_i = i;

    // interpret that child as long as possible
    loop {
      let (n_s, n_i) = self.interpret_node(n, best_i);
      if !n_s {
        break;
      }

      best_i = n_i;
    }

    // println!("star -> {}", best_i);
    return (true, best_i);
  }

  fn interpret_group(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // println!("group @ {}", self.print_chars(i));
    let mut last_i = i;

    // just interpret all children
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, last_i);
      if !n_s {
        // println!("group -> false");
        return (false, n_i);
      }

      last_i = n_i;
    }

    // println!("group -> true");
    return (true, last_i);
  }

  fn interpret_match_group(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // println!("interpret_match_group @ {}", i);
    // just interpret a group like normal
    // TODO: also save match data
    return self.interpret_group(node, i);
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

  pub fn eval(&self, str: String) -> RegExMatch {
    // create RegExMatch for the string
    let mut m = RegExMatch::new(str);
    m.interpret(&self.tree);

    return m;
  }
}
