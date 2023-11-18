use crate::parser::Parser;
use crate::parser::TreeNode;

pub struct RegEx {
  tree: TreeNode,
}

impl RegEx {
  pub fn new(expr: &str) -> Self {
    let mut parser = Parser::new(String::from(expr));
    return RegEx { tree: parser.parse() };
  }
}
