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
pub enum NodeType {
  Error,
  Empty,

  Word,
  Union,
  Star,
  Group,
  MatchGroup,
}

pub struct TreeNode {
  pub n_type: NodeType,
  pub image: Vec<char>,
  pub children: Vec<TreeNode>,
}

impl TreeNode {
  fn new(n_type: NodeType) -> Self {
    return TreeNode { n_type, image: vec![], children: vec![] };
  }

  fn add_child(&mut self, child: TreeNode) {
    if !matches!(child.n_type, NodeType::Empty) {
      self.children.push(child);
    }
  }

  fn add_children(&mut self, children: Vec<TreeNode>) {
    for c in children {
      self.add_child(c);
    }
  }

  fn make_group(children: Vec<TreeNode>, group_type: NodeType) -> TreeNode {
    let mut group = TreeNode::new(group_type);
    group.add_children(children);
    return group;
  }

  pub fn image_to_str(&self) -> String {
    let mut s = String::new();
    for i in 0..self.image.len() {
      s.push(self.image[i]);
    }
    return s;
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
    // parse
    return self.parse_root();
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

  fn parse_root(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // total -> expr eof
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        // println!("total -> expr eof");
        // create root node
        let mut root_node = TreeNode::new(NodeType::Group);

        // continue parsing
        root_node.add_children(self.parse_expr());
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

  fn parse_expr(&mut self) -> Vec<TreeNode> {
    match self.next_token.t_type {
      // expr -> seq union expr
      TokenType::Character | TokenType::LParen |
      TokenType::Union => {
        // println!("expr -> seq union expr");
        // create expr node
        // let mut expr_node = TreeNode::new(NodeType::Expression);
        let mut child_vec = vec![];

        // continue parsing
        let mut sequence = self.parse_seq(TreeNode::new(NodeType::Empty));

        let union_node;
        // println!("seq length!: {}", sequence.len());
        if sequence.len() == 1 {
          let first = sequence.pop().unwrap(); // pop to move [0] out of vec
          union_node = self.parse_union(first);
        }
        else {
          union_node = self.parse_union(TreeNode::make_group(sequence,
                                                             NodeType::Group));
        }

        child_vec.push(union_node);
        for n in self.parse_expr() {
          child_vec.push(n);
        }

        return child_vec;
      },
      // expr -> ε
      TokenType::RParen | TokenType::EOF => {
        // println!("expr -> ε");
        return vec![];
      },
      _ => {
        println!("syntax error: saw {:?} while parsing expr",
                 self.next_token.t_type);
        return vec![TreeNode::new(NodeType::Error)];
      },
    }
  }

  fn parse_seq(&mut self, mut prev: TreeNode) -> Vec<TreeNode> {
    match self.next_token.t_type {
      // seq -> atom star seq
      TokenType::Character | TokenType::LParen => {
        // println!("seq -> atom star seq");
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

        // create child vector
        let mut child_vec = vec![];
        // add previous and next nodes in sequence
        // only add a node to the vec if it isn't Empty
        if !matches!(prev.n_type, NodeType::Empty) {
          child_vec.push(prev);
        }
        for n in self.parse_seq(star_node) {
          if !matches!(n.n_type, NodeType::Empty) {
            child_vec.push(n);
          }
        }

        return child_vec;
      },
      // seq -> ε
      TokenType::Union | TokenType::RParen |
      TokenType::EOF => {
        // println!("seq -> ε");
        return vec![prev]; // return previous without modifying it
      },
      _ => {
        println!("syntax error: saw {:?} while parsing sequence",
                 self.next_token.t_type);
        return vec![TreeNode::new(NodeType::Error)];
      },
    }
  }

  fn parse_atom(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // atom -> charater
      TokenType::Character => {
        // println!("atom -> character");
        // create word node
        let mut word_node = TreeNode::new(NodeType::Word);
        word_node.image.push(self.next_token.image);

        // continue parsing
        self.eat(TokenType::Character);

        return word_node;
      },
      // atom -> ( expr )
      TokenType::LParen => {
        // println!("atom -> ( expr )");
        self.eat(TokenType::LParen);
        let expr_node = self.parse_expr();
        self.eat(TokenType::RParen);

        return TreeNode::make_group(expr_node, NodeType::MatchGroup);
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
        // println!("star -> *");
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
        // println!("star -> ε");
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
        // println!("union -> | expr");
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
        if expr_node.len() == 0 {
          union_node.add_child(TreeNode::new(NodeType::Word));
        }
        // if not empty, be normal
        else {
          union_node.add_children(expr_node);
        }

        return union_node;
      },
      // union -> ε
      TokenType::Character | TokenType::LParen |
      TokenType::RParen | TokenType::EOF => {
        // println!("union -> ε");
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
