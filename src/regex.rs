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
    let mut i = i;
    // try to match every char in word
    for c in &node.image {
      // reached the end of the string, can't possibly match
      if i >= self.chars.len() {
        return (false, i);
      }

      // break if mismatch
      if &self.chars[i] != c {
        return (false, i);
      }
      i += 1;
    }

    return (true, i); // nothing bad happened
  }

  fn interpret_union(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    let mut success = false;
    let mut best_i = i;
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, i);
      success = success | n_s;
      if n_s && n_i > best_i {
        best_i = n_i;
      }
    }

    return (success, best_i);
  }

  fn interpret_star(&self, node: &TreeNode, i: usize) -> (bool, usize) {
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

    return (true, best_i);
  }

  fn interpret_group(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    let mut last_i = i;

    // just interpret all children
    for n in &node.children {
      let (n_s, n_i) = self.interpret_node(n, i);
      if !n_s {
        return (false, n_i);
      }

      last_i = n_i;
    }

    return (true, last_i);
  }

  fn interpret_match_group(&self, node: &TreeNode, i: usize) -> (bool, usize) {
    // just interpret a group like normal
    // TODO: also save match data
    return self.interpret_group(node, i);
  }
}

pub struct RegEx {
  tree: TreeNode,
}

impl RegEx {
  pub fn new(expr: &str) -> Self {
    let mut parser = Parser::new(String::from(expr));
    return RegEx { tree: parser.parse() };
  }

  pub fn eval(&self, str: String) -> RegExMatch {
    // create RegExMatch for the string
    let mut m = RegExMatch::new(str);
    m.interpret(&self.tree);

    return m;
  }
}
