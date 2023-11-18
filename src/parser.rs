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
  Empty,

  Root,
  Expression,
  Sequence,
  Word,
  Grouping,
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

  fn add_child(&mut self, child: TreeNode) {
    if !matches!(child.n_type, NodeType::Empty) {
      self.children.push(child);
    }
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
    let tree = self.parse_total();
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
    else {
      // println!("ate {:?}: '{}'", t.t_type, t.image);
      self.next_token = self.scanner.scan_next();
    }
  }

  fn parse_total(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // total -> expr eof
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("total -> expr eof");
        // create root node
        let mut root_node = TreeNode::new(NodeType::Root);

        // continue parsing
        root_node.add_child(self.parse_expr());
        self.eat(TokenType::EOF);

        return root_node;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing total",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_expr(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // expr -> seq union expr
      TokenType::Character | TokenType::LParen |
      TokenType::Union => {
        println!("expr -> seq union expr");
        // create expr node
        let mut expr_node = TreeNode::new(NodeType::Expression);

        // continue parsing
        let seq_node = self.parse_seq(TreeNode::new(NodeType::Empty));
        let union_node = self.parse_union(seq_node);
        expr_node.add_child(union_node);
        expr_node.add_child(self.parse_expr()); // TODO: doesn't seem right

        return expr_node;
      },
      // expr -> ε
      TokenType::RParen | TokenType::EOF => {
        println!("expr -> ε");
        return TreeNode::new(NodeType::Empty);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing expr",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_seq(&mut self, mut prev: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // seq -> atom star seq
      TokenType::Character | TokenType::LParen => {
        println!("seq -> atom star seq");
        // continue parsing
        let atom_node = self.parse_atom();
        let star_node = self.parse_star(atom_node);

        // if previous node is word and star node remains a word
        // then instead of pushing a new node just expant that node's image
        if matches!(prev.n_type, NodeType::Word) &&
           matches!(star_node.n_type, NodeType::Word) {
          prev.image.push(star_node.image[0]);
          return self.parse_seq(prev);
        }
        // otherwise act like normal
        let mut seq_node = TreeNode::new(NodeType::Sequence);
        seq_node.add_child(prev);
        seq_node.add_child(self.parse_seq(star_node));
        return seq_node;
      },
      // seq -> ε
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("seq -> ε");
        return prev; // return previous without modifying it
      },
      _ => {
        println!("syntax error: saw {:?} while parsing sequence",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_atom(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // atom -> charater
      TokenType::Character => {
        println!("atom -> character");
        // create word node
        let mut word_node = TreeNode::new(NodeType::Word);
        word_node.image.push(self.next_token.image);

        // continue parsing
        self.eat(TokenType::Character);

        return word_node;
      },
      // atom -> ( expr )
      TokenType::LParen => {
        println!("atom -> ( expr )");
        // TODO: create grouping node

        self.eat(TokenType::LParen);
        let expr_node = self.parse_expr();
        self.eat(TokenType::RParen);

        return expr_node;
      },
      _ => {
        println!("syntax error: saw {:?} while parsing atom",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_star(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // star -> *
      TokenType::Star => {
        println!("star -> *");
        // create star node
        let mut star_node = TreeNode::new(NodeType::Star);
        star_node.add_child(lhs);

        // continue parsing
        self.eat(TokenType::Star);

        return star_node;
      },
      // star -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        println!("star -> ε");
        return lhs; // return lhs unmodified
      },
      _ => {
        println!("syntax error: saw {:?} while parsing star",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_union(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      // union -> | expr
      TokenType::Union => {
        println!("union -> | expr");
        // create union node
        let mut union_node = TreeNode::new(NodeType::Union);
        // if lhs is empty replace it with a 0-length word
        if matches!(lhs.n_type, NodeType::Empty) {
          union_node.add_child(TreeNode::new(NodeType::Word));
        }
        // if not empty, be normal
        else {
          union_node.add_child(lhs);
        }

        // continue parsing
        self.eat(TokenType::Union);
        let expr_node = self.parse_expr();

        // if expression is empty replace it with a 0-length word
        // otherwise it will be culled and you won't be able to match (a|b|)
        if matches!(expr_node.n_type, NodeType::Empty) {
          union_node.add_child(TreeNode::new(NodeType::Word));
        }
        // if not empty, be normal
        else {
          union_node.add_child(expr_node);
        }

        return union_node;
      },
      // union -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::RParen | TokenType::EOF => {
        println!("union -> ε");
        return lhs; // return lhs unmodified
      },
      _ => {
        println!("syntax error: saw {:?} while parsing union",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }
}

// DEBUG
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
  else if matches!(node.n_type, NodeType::Word) {
    print!("(ε)");
  }
  println!("]");

  for n in &node.children {
    print_node(n, depth + 1);
  }
}
