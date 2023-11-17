use crate::nfa::NFA;
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

enum NodeType {
  Error,
  Expression,
  Character,
  Group,
  Union,
  Star,
  End,
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
  nfa: NFA,
}

impl Parser {
  fn new(input: String) -> Self {
    let scanner = Scanner::new(input);

    return Parser {
      scanner,
      next_token: Token::new(TokenType::Error),
      nfa: NFA::new(),
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

  fn parse(&mut self) {
    // point to first character
    self.next_token = self.scanner.scan_next();
    self.parse_expr();
  }

  fn parse_expr(&mut self) {
    match self.next_token.t_type {
      TokenType::Slash => {
        println!("expr -> / concat /");
        self.eat(TokenType::Slash);
        self.parse_concat();
        self.eat(TokenType::Slash);
      },
      _ => {
        println!("syntax error: while parsing expr");
      },
    }
  }

  fn parse_concat(&mut self) {
    match self.next_token.t_type {
      TokenType::Character => {
        println!("concat -> char star concat_tail");
        self.eat(TokenType::Character);
        self.parse_star();
        self.parse_concat_tail();
      },
      TokenType::LParen => {
        println!("concat -> group star concat_tail");
        self.parse_group();
        self.parse_star();
        self.parse_concat_tail();
      },
      _ => {
        println!("syntax error: while parsing concat");
      },
    }
  }

  fn parse_concat_tail(&mut self) {
    match self.next_token.t_type {
      TokenType::Character => {
        println!("concat_tail -> concat concat_tail");
        self.parse_concat();
      },
      TokenType::Union => {
        println!("concat_tail -> union concat_tail");
        self.parse_union();
        self.parse_concat_tail();
      }
      TokenType::RParen | TokenType::Slash => {
        println!("char_tail -> ε");
      },
      _ => {
        println!("syntax error: while parsing char_tail");
      },
    }
  }

  fn parse_star(&mut self) {
    match self.next_token.t_type {
      TokenType::Star => {
        println!("star -> *");
        self.eat(TokenType::Star);
      },
      TokenType::Character | TokenType::LParen |
      TokenType::Union | TokenType::RParen |
      TokenType::Slash => {
        println!("star -> ε");
      },
      _ => {
        println!("syntax error: while parsing star");
      },
    }
  }

  fn parse_group(&mut self) {
    match self.next_token.t_type {
      TokenType::LParen => {
        println!("group -> ( concat )");
        self.eat(TokenType::LParen);
        self.parse_concat();
        self.eat(TokenType::RParen);
      },
      _ => {
        println!("syntax error: while parsing group");
      },
    }
  }

  fn parse_union(&mut self) {
    match self.next_token.t_type {
      TokenType::Union => {
        println!("union -> | concat");
        self.eat(TokenType::Union);
        self.parse_concat();
      },
      _ => {
        println!("syntax error: while parsing union");
      },
    }
  }
}

pub struct Compiler {
  parser: Parser,
}

impl Compiler {
  pub fn new(input: String) -> Self {
    let parser = Parser::new(input);
    return Compiler { parser };
  }

  pub fn compile(&mut self) {
    self.parser.parse();
  }
}
