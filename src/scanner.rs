#[derive(Debug)]
pub enum TokenType {
  Error,
  Character,
  Union,
  Star,
  LParen,
  RParen,
  LBracket,
  RBracket,
  Caret,
  Question,
  Plus,
  Range,
  EOF,
}

#[derive(Clone)]
pub struct CharRange {
  pub min: u32,
  pub max: u32,
  pub negate: bool,
}

impl CharRange {
  pub fn new(min: u32, max: u32, negate: bool) -> Self {
    return CharRange { min, max, negate };
  }

  pub fn includes_char(&self, c: char) -> bool {
    let u = c as u32;

    if !self.negate { u >= self.min && u <= self.max }
    else { u < self.min || u > self.max }
  }
}

struct PerlCC {
  // Empty
}

impl PerlCC {
  // NOTE: these character classes are for ASCII ranges only
  const DIGIT: &[CharRange] = &[
    CharRange { min: '0' as u32, max: '9' as u32, negate: false },
  ];

  const NOT_DIGIT: &[CharRange] = &[
    CharRange { min: '0' as u32, max: '9' as u32, negate: true },
  ];

  const WORD: &[CharRange] = &[
    CharRange { min: 'a' as u32, max: 'z' as u32, negate: false },
    CharRange { min: 'A' as u32, max: 'Z' as u32, negate: false },
    CharRange { min: '0' as u32, max: '9' as u32, negate: false },
    CharRange { min: '_' as u32, max: '_' as u32, negate: false },
  ];

  const NOT_WORD: &[CharRange] = &[
    CharRange { min: 'a' as u32, max: 'z' as u32, negate: true },
    CharRange { min: 'A' as u32, max: 'Z' as u32, negate: true },
    CharRange { min: '0' as u32, max: '9' as u32, negate: true },
    CharRange { min: '_' as u32, max: '_' as u32, negate: true },
  ];

  const WHITESPACE: &[CharRange] = &[
    // NOTE: this includes \f, which some versions of Perl do not
    CharRange { min: 0x000A, max: 0x000D, negate: false },
  ];

  const NOT_WHITESPACE: &[CharRange] = &[
    CharRange { min: 0x000A, max: 0x000D, negate: true },
  ];

  const NOT_NEWLINE: &[CharRange] = &[
    CharRange { min: '\n' as u32, max: '\n' as u32, negate: true },
  ];

  // TODO: \h, \H, \v, \V
}

pub struct Token {
  pub t_type: TokenType,
  pub image: char,
  pub range: Vec<CharRange>,
}

impl Token {
  pub fn new(t_type: TokenType, image: char) -> Self {
    return Token {
      t_type,
      image,
      range: vec![CharRange::new(0x0000, 0x0000, true)],
    };
  }
}

pub struct Scanner {
  // input: String,
  chars: Vec<char>,
  index: usize,
}

enum EscapeType {
  Basic,
  UnicodeHex,
  AsciiDec,
}

fn char_to_hex(h: char) -> u32 {
  match h {
    '0' => 0x0, '1' => 0x1, '2' => 0x2, '3' => 0x3, '4' => 0x4, '5' => 0x5,
    '6' => 0x6, '7' => 0x7, '8' => 0x8, '9' => 0x9, 'a' => 0xa, 'b' => 0xb,
    'c' => 0xc, 'd' => 0xd, 'e' => 0xe, 'f' => 0xf, _ => 0x0,
  }
}

impl Scanner {
  pub fn new(input: &String) -> Self {
    let chars = input.chars().collect();
    return Scanner {
      chars,
      index: 0usize,
    };
  }

  pub fn scan_next(&mut self) -> Token {
    let t = match self.chars.get(self.index) {
      Some(c) => self.char_to_token(*c),
      None => Token::new(TokenType::EOF, '\0'),
    };

    self.index += 1;

    return t;
  }

  fn handle_escape(&mut self) -> Token {
    let mut escape_len = 0;
    let mut escape_type = EscapeType::Basic;
    let mut unicode_hex: u32 = 0x0;
    loop {
      // modified scan_next procedure
      self.index += 1;
      let mut c;
      match self.chars.get(self.index) {
        Some(nc) => { c = nc; },
        None => {
          println!("lexical error: saw EOF while parsing escape sequence");
          return Token::new(TokenType::Error, '\0');
        },
      }

      // handle one-character escape sequences
      if escape_len == 0 {
        match c {
          // begin unicode sequence
          'u' => {
            escape_len += 1;
          }
          // TODO: ascii number sequences
          // "basic" escapes
          't' => { return Token::new(TokenType::Character, '\t'); },
          'n' => { return Token::new(TokenType::Character, '\n'); },
          'r' => { return Token::new(TokenType::Character, '\r'); },
          // Perl character classes
          'd' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::DIGIT),
            };
          },
          'D' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::NOT_DIGIT),
            };
          },
          'w' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::WORD),
            };
          },
          'W' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::NOT_WORD),
            };
          },
          's' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::WHITESPACE),
            };
          },
          'S' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::NOT_WHITESPACE),
            };
          },
          'N' => {
            return Token {
              t_type: TokenType::Range,
              image: '\0',
              range: Vec::from(PerlCC::NOT_NEWLINE),
            };
          }
          // no special meaning, just return character
          // NOTE: this means something like \y - which isn't a valid escape
          // sequence - would parse as "y" instead of throwing an error
          _ => { return Token::new(TokenType::Character, *c); },
        };
      }
      // handle multi-character (unicode) escape sequences
      else {
        match c {
          '0'..='9' | 'a'..='f'  => {
            unicode_hex = unicode_hex << 4;
            unicode_hex |= char_to_hex(*c);
            escape_len += 1;
          },
          _ => {
            println!("lexical error: saw '{}' while parsing unicode hex", *c);
            return Token::new(TokenType::Error, *c);
          },
        }

        // return once code has been finished
        if escape_len == 5 { // 'u' + 4 hex characters
          return match char::from_u32(unicode_hex) {
            Some(u) => Token::new(TokenType::Character, u),
            None => {
              println!("lexical error: hex is not a valid unicode character");
              return Token::new(TokenType::Error, '\0');
            },
          };
        }
      }
    }
  }

  fn char_to_token(&mut self, c: char) -> Token {
    match c {
      '|' => Token::new(TokenType::Union, c),
      '*' => Token::new(TokenType::Star, c),
      '(' => Token::new(TokenType::LParen, c),
      ')' => Token::new(TokenType::RParen, c),
      '[' => Token::new(TokenType::LBracket, c),
      ']' => Token::new(TokenType::RBracket, c),
      '^' => Token::new(TokenType::Caret, c),
      '?' => Token::new(TokenType::Question, c),
      '+' => Token::new(TokenType::Plus, c),
      '.' => Token {
        t_type: TokenType::Range,
        image: c,
        range: vec![CharRange::new(0x0000, 0xFFFF, false)],
      },
      '\\' => self.handle_escape(),
      _ => Token::new(TokenType::Character, c),
    }
  }
}
