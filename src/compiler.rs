use crate::nfa::State;

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

struct NFAModule {
  heads: Vec<State>,
  tails: Vec<State>,
}

#[derive(Debug)]
enum NodeType {
  Error,
  Expression,
  Character,
  Group,
  Union,
  Star,
}

struct TreeNode {
  n_type: NodeType,
  image: char,
  children: Vec<TreeNode>,
}

impl TreeNode {
  fn new(n_type: NodeType) -> Self {
    return TreeNode { n_type, image: '\0', children: vec![] };
  }
}

struct Parser {
  scanner: Scanner,
  next_token: Token,
}

impl Parser {
  fn new(input: String) -> Self {
    let scanner = Scanner::new(input);

    return Parser {
      scanner,
      next_token: Token::new(TokenType::Error),
    };
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
      println!("ate {:?}: '{}'", t.t_type, t.image);
      self.next_token = self.scanner.scan_next();
    }
  }

  fn parse(&mut self) -> TreeNode {
    // point to first character
    self.next_token = self.scanner.scan_next();
    return self.parse_expr();
  }

  fn parse_expr(&mut self) -> TreeNode {
    match self.next_token.t_type {
      TokenType::Slash => {
        println!("expr -> / concat /");
        // create new expression node
        let mut expr_node = TreeNode::new(NodeType::Expression);

        // continue parsing
        self.eat(TokenType::Slash);
        expr_node.children.push(self.parse_concat());
        self.eat(TokenType::Slash);

        return expr_node;
      },
      _ => {
        println!("syntax error: while parsing expr");
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_concat(&mut self) -> TreeNode {
    match self.next_token.t_type {
      TokenType::Character => {
        println!("concat -> char star concat_tail");
        // create new character node
        let mut char_node = TreeNode::new(NodeType::Character);
        char_node.image = self.next_token.image;

        // continue parsing
        self.eat(TokenType::Character);
        let star_node = self.parse_star(char_node);
        return self.parse_concat_tail(star_node);
      },
      TokenType::LParen => {
        println!("concat -> group star concat_tail");
        let group_node = self.parse_group();
        let star_node = self.parse_star(group_node);
        return self.parse_concat_tail(star_node);
      },
      TokenType::Union => {
        println!("concat -> union concat_tail");
        let union_node = self.parse_union(TreeNode::new(NodeType::Character));
        return self.parse_concat_tail(union_node);
      },
      _ => {
        println!("syntax error: while parsing concat");
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_concat_tail(&mut self, mut lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      TokenType::Character => {
        println!("concat_tail -> concat concat_tail");
        lhs.children.push(self.parse_concat());
        return lhs;
      },
      TokenType::Union => {
        println!("concat_tail -> union concat_tail");
        let union_node = self.parse_union(lhs);
        return self.parse_concat_tail(union_node);
      }
      TokenType::RParen | TokenType::Slash => {
        println!("char_tail -> ε");
        return lhs;
      },
      _ => {
        println!("syntax error: while parsing char_tail");
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_star(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      TokenType::Star => {
        println!("star -> *");
        self.eat(TokenType::Star);

        // create star node
        let mut star_node = TreeNode::new(NodeType::Star);
        star_node.children.push(lhs);
        return star_node;
      },
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::Slash => {
        println!("star -> ε");

        // rhs is unchanged
        return lhs;
      },
      _ => {
        println!("syntax error: while parsing star");
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_group(&mut self) -> TreeNode {
    match self.next_token.t_type {
      TokenType::LParen => {
        println!("group -> ( concat )");
        // create group node
        let mut group_node = TreeNode::new(NodeType::Group);

        // continue parsing
        self.eat(TokenType::LParen);
        group_node.children.push(self.parse_concat());
        self.eat(TokenType::RParen);

        return group_node;
      },
      _ => {
        println!("syntax error: while parsing group");
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_union(&mut self, lhs: TreeNode) -> TreeNode {
    match self.next_token.t_type {
      TokenType::Union => {
        println!("union -> | concat");
        // create union node
        let mut union_node = TreeNode::new(NodeType::Union);
        union_node.children.push(lhs);

        // continue parsing
        self.eat(TokenType::Union);
        union_node.children.push(self.parse_concat());

        return union_node;
      },
      _ => {
        println!("syntax error: while parsing union");
        return TreeNode::new(NodeType::Error);
      },
    }
  }
}

// TODO: redundant
pub struct Compiler {
  parser: Parser,
}

impl Compiler {
  pub fn new(input: String) -> Self {
    let parser = Parser::new(input);
    return Compiler { parser };
  }

  pub fn compile(&mut self) {
    let tree = self.parser.parse();
    print_node(tree, 0);
  }
}

// TEMP DEBUG
fn print_node(node: TreeNode, depth: i32) {
  let mut depth_str = String::new();
  for _ in 0..depth {
    depth_str.push_str("  ");
  }

  print!("{}", depth_str);
  print!("[{:?}", node.n_type);
  if node.image != '\0' {
    print!("({})", node.image);
  }
  println!("]");

  for n in node.children {
    print_node(n, depth + 1);
  }
}
