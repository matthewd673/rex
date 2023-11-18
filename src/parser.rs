#[derive(Debug)]
pub enum TokenType {
  Error,
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
    self.parse_expr();
    // print_node(&tree, 0);
    // return tree;
    //
    return TreeNode::new(NodeType::Error);
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

  fn parse_total(&mut self) {
    match self.next_token.t_type {
      // total -> expr eof
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("total -> expr eof");
        self.parse_expr();
        self.eat(TokenType::EOF);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing total",
                 self.next_token.t_type);
      },
    }
  }

  fn parse_expr(&mut self) {
    match self.next_token.t_type {
      // expr -> seq union expr
      TokenType::Character | TokenType::LParen |
      TokenType::Union => {
        println!("expr -> seq union expr");
        self.parse_seq();
        self.parse_union();
        self.parse_expr();
      },
      // expr -> ε
      TokenType::RParen | TokenType::EOF => {
        println!("expr -> ε");
        // empty
      },
      _ => {
        println!("syntax error: saw {:?} while parsing expr",
                 self.next_token.t_type);
        // return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_seq(&mut self) {
    match self.next_token.t_type {
      // seq -> atom star seq
      TokenType::Character | TokenType::LParen => {
        println!("seq -> atom star seq");
        self.parse_atom();
        self.parse_star();
        self.parse_seq();
      },
      // seq -> ε
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("seq -> ε");
        // empty
      },
      _ => {
        println!("syntax error: saw {:?} while parsing sequence",
                 self.next_token.t_type);
      },
    }
  }

  fn parse_atom(&mut self) {
    match self.next_token.t_type {
      // atom -> charater
      TokenType::Character => {
        println!("atom -> character");
        self.eat(TokenType::Character);
      },
      // atom -> ( expr )
      TokenType::LParen => {
        println!("atom -> ( expr )");
        self.eat(TokenType::LParen);
        self.parse_expr();
        self.eat(TokenType::RParen);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing atom",
                 self.next_token.t_type);
      },
    }
  }

  fn parse_star(&mut self) {
    match self.next_token.t_type {
      // star -> *
      TokenType::Star => {
        println!("star -> *");
        self.eat(TokenType::Star);
      },
      // star -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("star -> ε");
        // empty
      },
      _ => {
        println!("syntax error: saw {:?} while parsing star",
                 self.next_token.t_type);
      },
    }
  }

  fn parse_union(&mut self) {
    match self.next_token.t_type {
      // union -> |
      TokenType::Union => {
        println!("union -> |");
        self.eat(TokenType::Union);
      },
      // union -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::RParen | TokenType::EOF => {
        println!("union -> ε");
        // empty
      },
      _ => {
        println!("syntax error: saw {:?} while parsing union",
                 self.next_token.t_type);
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
