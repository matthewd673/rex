use std::collections::VecDeque;

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
  fn new_Error() -> Self {
    return Token { t_type: TokenType::Error, image: '\0' };
  }

  fn new_EOF() -> Self {
    return Token { t_type: TokenType::EOF, image: '\0' };
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

  // pub fn scan(&self) -> VecDeque<Token> {
    // let mut tokens = VecDeque::new();

    // tokenize all chars in the string
    // let mut chars = self.input.chars();
    // loop {
      // let next_char = chars.next();
      // match next_char {
        // Some(c) => { tokens.push_back(char_to_token(c)); },
        // None => { break; },
      // }
    // }

    // // always finish with EOF
    // tokens.push_back(Token { t_type: TokenType::EOF, image: '\0' });

    // return tokens;
  // }

  fn scan_next(&mut self) -> Token {
    let t = match self.chars.get(self.index) {
      Some(c) => char_to_token(*c),
      None => Token::new_EOF(),
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

struct Parser {
  scanner: Scanner,
  next_token: Token,
}

impl Parser {
  fn new(input: String) -> Self {
    let scanner = Scanner::new(input);

    return Parser { scanner, next_token: Token::new_Error() };
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
    self.next_token = self.scanner.scan_next();
    self.parse_expr();
  }

  fn parse_expr(&mut self) {
    match self.next_token.t_type {
      TokenType::Character => {
        println!("expr -> C ET");
        self.parse_char();
        self.parse_expr_tail();
      },
      TokenType::LParen => {
        println!("expr -> ( E ) ET");
        self.eat(TokenType::LParen);
        self.parse_char();
        self.eat(TokenType::RParen);
        self.parse_expr_tail();
      }
      _ => {
        println!("syntax error: while parsing expr");
      },
    }
  }

  fn parse_expr_tail(&mut self) {
    match self.next_token.t_type {
      TokenType::Character | TokenType::LParen => {
        println!("expr_tail -> E ET");
        self.parse_expr();
        self.parse_expr_tail();
      },
      TokenType::Union => {
        println!("expr_tail -> | ET");
        self.eat(TokenType::Union);
        self.parse_expr_tail();
      },
      TokenType::Star => {
        println!("expr_tail -> * ET");
        self.eat(TokenType::Star);
        self.parse_expr_tail();
      },
      TokenType::RParen | TokenType::EOF => {
        println!("expr_tail -> ε");
      },
      _ => {
        println!("syntax error: while parsing expr_tail");
      },
    }
  }

  fn parse_char(&mut self) {
    // TODO: formalize into match
    self.eat(TokenType::Character);
    self.parse_char_tail();
  }

  fn parse_char_tail(&self) {
    match self.next_token.t_type {
      TokenType::Character | TokenType::RParen | TokenType::Union |
      TokenType::Star | TokenType::EOF => {
        println!("char_tail -> ε");
      },
      _ => {
        println!("syntax error: while parsing char_tail");
      },
    }
  }
}

pub struct Compiler {
  parser: Parser,
}

impl Compiler {
  pub fn new(input: String) -> Self {
    let mut parser = Parser::new(input);
    parser.parse();

    return Compiler { parser };
  }
}
