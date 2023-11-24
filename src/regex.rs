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

  fn interpret(&mut self, tree: TreeNode) {
    self.success = self.interpret_node(&tree, 0);
  }

  fn interpret_node(&self, node: &TreeNode, i: usize) -> bool {
    match &node.n_type {
      NodeType::Word => self.interpret_word(node, i),
      NodeType::Union => self.interpret_union(node, i),
      NodeType::Star => self.interpret_star(node, i),
      NodeType::Group => self.interpret_group(node, i),
      NodeType::MatchGroup => self.interpret_match_group(node, i),
      _ => {
        println!("runtime error: unknown node type {:?}", node.n_type);
        return false;
      },
    }
  }

  fn interpret_word(&self, node: &TreeNode, i: usize) -> bool {
    let mut i = i;
    // try to match every char in word
    for c in &node.image {
      // reached the end of the string, can't possibly match
      if i >= self.chars.len() {
        return false;
      }

      // break if mismatch
      if &self.chars[i] != c {
        return false;
      }
      i += 1;
    }

    return true; // nothing bad happened
  }

  fn interpret_union(&self, node: &TreeNode, i: usize) -> bool {
    let mut success = false;
    for n in &node.children {
      success = success | self.interpret_node(n, i);
    }

    return success;
  }

  fn interpret_star(&self, node: &TreeNode, i: usize) -> bool {
    // there will only ever be one child
    let n = &node.children[0];

    // interpret that child as long as possible
    loop {
      if !self.interpret_node(n, i) {
        break;
      }
    }

    return true;
  }

  fn interpret_group(&self, node: &TreeNode, i: usize) -> bool {
    // just interpret all children
    for n in &node.children {
      let success = self.interpret_node(n, i);
      if !success {
        return false;
      }
    }

    return true;
  }

  fn interpret_match_group(&self, node: &TreeNode, i: usize) -> bool {
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

  pub fn eval(self, str: String) -> RegExMatch {
    // create RegExMatch for the string
    let mut m = RegExMatch::new(str);
    m.interpret(self.tree);

    return m;
  }
}
