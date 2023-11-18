#[derive(Debug)]
pub enum TokenType {
  Error,
  Slash,
  Character,
  Union,
  Star,
  LParen,
  RParen,
  EOF,
}

pub struct Token {
  pub t_type: TokenType,
  pub image: char,
}

impl Token {
  fn new(t_type: TokenType) -> Self {
    return Token { t_type, image: '\0' };
  }
}

struct Scanner {
  // input: String,
  chars: Vec<char>,
  index: usize,
}

impl Scanner {
  fn new(input: String) -> Self {
    let chars = input.chars().collect();
    return Scanner { chars, index: 0usize };
  }

  fn scan_next(&mut self) -> Token {
    let t = match self.chars.get(self.index) {
      Some(c) => char_to_token(*c),
      None => Token::new(TokenType::EOF),
    };

    self.index += 1;

    return t;
  }
}

// TODO: escaping
fn char_to_token(c: char) -> Token {
  match c {
    '/' => Token { t_type: TokenType::Slash, image: c },
    '|' => Token { t_type: TokenType::Union, image: c },
    '*' => Token { t_type: TokenType::Star, image: c },
    '(' => Token { t_type: TokenType::LParen, image: c },
    ')' => Token { t_type: TokenType::RParen, image: c },
    _ => Token { t_type: TokenType::Character, image: c },
  }
}

#[derive(Debug)]
enum NodeType {
  Error,
  Expression,
  Sequence,
  Group,
  Union,
  Star,
}

pub struct TreeNode {
  n_type: NodeType,
  image: Vec<char>,
  children: Vec<TreeNode>,
  success: bool,
}

impl TreeNode {
  fn new(n_type: NodeType) -> Self {
    return TreeNode { n_type, image: vec![], children: vec![], success: false };
  }
}

pub struct Parser {
  scanner: Scanner,
  next_token: Token,
}

impl Parser {
  pub fn new(input: String) -> Self {
    let scanner = Scanner::new(input);

    return Parser {
      scanner,
      next_token: Token::new(TokenType::Error),
    };
  }

  pub fn parse(&mut self) -> TreeNode {
    // point to first character
    self.next_token = self.scanner.scan_next();
    // TODO: temp debug printing
    let tree = self.parse_expr();
    print_node(&tree, 0);
    return tree;
  }

  fn eat(&mut self, expected: TokenType) {
    let t = &self.next_token;

    if !matches!(&t.t_type, expected) {
      println!("syntax error: expected {:?}, saw {:?}",
               t.t_type,
               expected
               );
    }
    // TODO: temp
    else {
      // println!("ate {:?}: '{}'", t.t_type, t.image);
      self.next_token = self.scanner.scan_next();
    }
  }

  fn parse_expr(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // expr -> / concat /
      TokenType::Slash => {
        // create new expression node
        let mut expr_node = TreeNode::new(NodeType::Expression);

        // continue parsing
        self.eat(TokenType::Slash);
        expr_node.children.push(self.parse_concat(None));
        self.eat(TokenType::Slash);

        return expr_node;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing expr",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_concat(&mut self, seq: Option<TreeNode>) -> TreeNode {
    match self.next_token.t_type {
      // concat -> char star concat_tail
      TokenType::Character => {
        // append to existing sequence or create a new one
        let mut seq_node;
        match seq {
          Some(s) => { seq_node = s; },
          None => { seq_node = TreeNode::new(NodeType::Sequence); },
        }
        seq_node.image.push(self.next_token.image);

        // continue parsing
        self.eat(TokenType::Character);
        let star_node = self.parse_star(seq_node);
        return self.parse_concat_tail(star_node);
      },
      // concat -> group star concat_tail
      TokenType::LParen => {
        let group_node = self.parse_group();
        let star_node = self.parse_star(group_node);
        return self.parse_concat_tail(star_node);
      },
      // concat -> union concat_tail
      TokenType::Union => {
        let union_node = self.parse_union(TreeNode::new(NodeType::Sequence));
        return self.parse_concat_tail(union_node);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing concat",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_concat_tail(&mut self, mut lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // concat_tail -> concat concat_tail
      TokenType::Character => {
        // either continue appending to a sequence
        // or start a new sequence and make it a child of lhs
        match lhs.n_type {
          NodeType::Sequence => {
            return self.parse_concat(Some(lhs));
          },
          _ => {
            let concat_node = self.parse_concat(None);
            lhs.children.push(concat_node);
            return lhs;
          }
        }
      },
      // concat_tail -> union concat_tail
      TokenType::Union => {
        let union_node = self.parse_union(lhs);
        return self.parse_concat_tail(union_node);
      }
      // char_tail -> ε
      TokenType::RParen | TokenType::Slash => {
        return lhs;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing char_tail",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_star(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // star -> *
      TokenType::Star => {
        self.eat(TokenType::Star);

        // create star node
        let mut star_node = TreeNode::new(NodeType::Star);
        star_node.children.push(lhs);
        return star_node;
      },
      // star -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::Slash => {
        // rhs is unchanged
        return lhs;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing star",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_group(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // group -> ( concat )
      TokenType::LParen => {
        // create group node
        let mut group_node = TreeNode::new(NodeType::Group);

        // continue parsing
        self.eat(TokenType::LParen);
        group_node.children.push(self.parse_concat(None));
        self.eat(TokenType::RParen);

        return group_node;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing group",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  // TODO: production should be union -> concat*_tail*
  fn parse_union(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // union -> | concat
      TokenType::Union => {
        // create union node
        let mut union_node = TreeNode::new(NodeType::Union);
        union_node.children.push(lhs);

        // continue parsing
        self.eat(TokenType::Union);
        union_node.children.push(self.parse_concat(None));

        return union_node;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing union",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }
}

// TEMP DEBUG
fn print_node(node: &TreeNode, depth: i32) {
  let mut depth_str = String::new();
  for _ in 0..depth {
    depth_str.push_str("  ");
  }

  print!("{}", depth_str);
  print!("[{:?}", node.n_type);
  if node.image.len() > 0 {
    print!("(");
    for c in &node.image {
      print!("{}", c);
    }
    print!(")");
  }
  else if node.image.len() == 0 && matches!(node.n_type, NodeType::Sequence) {
    print!("(ε)");
  }
  println!("]");

  for n in &node.children {
    print_node(n, depth + 1);
  }
}
