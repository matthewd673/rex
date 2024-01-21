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
  chars: Vec<char>,
  index: usize,
}

enum EscapeType {
  Basic,
  UnicodeHex,
  AsciiDec,
  AsciiHex,
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
    let mut unicode_code: u32 = 0x0;
    let mut ascii_code: u32 = 0x0;
    loop {
      // modified scan_next procedure
      self.index += 1;
      let mut c;
      match self.chars.get(self.index) {
        Some(nc) => { c = nc; },
        None => {
          if escape_len == 0 {
            println!("lexical error: saw EOF while parsing escape sequence");
            return Token::new(TokenType::Error, '\0');
          }
          // EOF can be handled by escape code parsers
          else {
            c = &'\0';
          }
        },
      }

      // handle one-character escape sequences
      if escape_len == 0 {
        match c {
          // begin unicode hex sequence
          'u' => {
            escape_type = EscapeType::UnicodeHex;
            escape_len += 1;
          },
          'x' => {
            escape_type = EscapeType::AsciiHex;
            escape_len += 1;
          },
          '0'..='9' => {
            escape_type = EscapeType::AsciiDec;
            escape_len += 1;
            // un-consume character so it can be handled
            self.index -= 1;
          },
          // "basic" escapes
          't' => { return Token::new(TokenType::Character, '\t'); },
          'n' => { return Token::new(TokenType::Character, '\n'); },
          'v' => { return Token::new(TokenType::Character, '\x0b'); },
          'f' => { return Token::new(TokenType::Character, '\x0c'); },
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
          // TODO: this means something like \y - which isn't a valid escape
          //   sequence - would parse as "y" instead of throwing an error
          _ => { return Token::new(TokenType::Character, *c); },
        };
      }
      // handle multi-character (unicode) escape sequences
      else {
        let mut completed_code = false;

        match escape_type {
          EscapeType::Basic => {
            // Empty - this will never occur
          },
          EscapeType::UnicodeHex => 'unicode_hex_match: {
            if escape_len == 5 {
              completed_code = true;
              break 'unicode_hex_match;
            }
            match c {
              '0'..='9' | 'a'..='f' => {
                unicode_code = unicode_code << 4;
                unicode_code |= char_to_hex(*c);
                escape_len += 1;
              },
              _ => {
                println!("lexical error: saw '{}' while parsing unicode hex",
                         *c);
                return Token::new(TokenType::Error, *c);
              },
            }
          },
          EscapeType::AsciiHex => 'ascii_hex_match: {
            // sequence must be done within 3, un-consume and break
            if escape_len == 3 {
              completed_code = true;
              break 'ascii_hex_match;
            }
            match c {
              '0'..='9' | 'a'..='f' => {
                let mut temp_ascii_code = ascii_code << 4;
                temp_ascii_code |= char_to_hex(*c);

                // if we reach the max value within 3 digits (e.g.: "234")
                // then un-consume the current character and complete the code
                if temp_ascii_code > 127 {
                  completed_code = true;
                  break 'ascii_hex_match;
                }

                ascii_code = temp_ascii_code;
                escape_len += 1;
              },
              // otherwise, just un-consume character and break early
              _ => {
                completed_code = true;
              }
            }
          },
          EscapeType::AsciiDec => 'ascii_dec_match: {
            if escape_len == 3{
              completed_code = true;
              break 'ascii_dec_match;
            }
            match c {
              '0'..='9' => {
                let mut temp_ascii_code = ascii_code * 10;
                temp_ascii_code += char_to_hex(*c); // this does char->digit too

                // if we reach the max value, un-consume and break
                if temp_ascii_code > 127 {
                  completed_code = true;
                  break 'ascii_dec_match;
                }

                ascii_code = temp_ascii_code;
                escape_len += 1;
              }
              // otherwise, just un-consume character and break early
              _ => {
                completed_code = true;
              }
            }
          }
        }

        // return once code has been finished
        if completed_code {
          // un-consume the last character - it was not part of the code
          self.index -= 1;

          // convert code to unicode char
          match escape_type {
            EscapeType::UnicodeHex => {
              return match char::from_u32(unicode_code) {
                Some(u) => Token::new(TokenType::Character, u),
                None => {
                  println!("lexical error: invalid unicode escape code");
                  return Token::new(TokenType::Error, '\0');
                },
              };
            },
            EscapeType::AsciiHex | EscapeType::AsciiDec => {
              return match char::from_u32(ascii_code) {
                Some(a) => Token::new(TokenType::Character, a),
                // invalid characters should already be caught in the loop
                None => {
                  println!("lexical error: invalid ascii escape code");
                  return Token::new(TokenType::Error, '\0');
                }
              };
            },
            _ => {
              // Empty - this should never happen
            },
          }
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
        // TODO: in Perl this exludes '\n' by default
        range: vec![CharRange::new(0x0000, 0xFFFF, false)],
      },
      '\\' => self.handle_escape(),
      _ => Token::new(TokenType::Character, c),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn scan_all(s: &mut Scanner) -> Vec<Token> {
    let mut tokens = vec![];
    while s.index <= s.chars.len() {
      tokens.push(s.scan_next());
    }

    return tokens;
  }

  fn test_token_types(tokens: &Vec<Token>, t_types: Vec<TokenType>) {
    assert_eq!(tokens.len(), t_types.len());

    for i in 0..tokens.len() {
      let a = &tokens[i].t_type;
      let b = &t_types[i];
      assert!(matches!(a, b));
    }
  }

  fn test_token_images(tokens: &Vec<Token>, images: Vec<char>) {
    assert_eq!(tokens.len(), images.len());

    for i in 0..tokens.len() {
      let a = &tokens[i].image;
      let b = &images[i];
      assert_eq!(a, b);
    }
  }

  #[test]
  fn scan_characters() {
    let mut s = Scanner::new(&String::from("abc"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', 'b', 'c', '\0']);
  }

  #[test]
  fn scan_escaped_backslash() {
    let mut s = Scanner::new(&String::from("\\\\"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['\\', '\0']);
  }

  #[test]
  fn scan_escapes_in_sequence() {
    let mut s = Scanner::new(&String::from("\\\\\\t"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['\\', '\t', '\0']);
  }

  #[test]
  fn scan_unicode_escape() {
    let mut s = Scanner::new(&String::from("\\u2603"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['\u{2603}', '\0']);
  }

  #[test]
  fn scan_common_escapes() {
    let mut s = Scanner::new(&String::from("\\t\\n\\v\\f\\r"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['\t', '\n', '\x0b', '\x0c', '\r', '\0']);
  }

  #[test]
  fn scan_ascii_digit_escapes() {
    let mut s = Scanner::new(&String::from("\\97\\0\\32a"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                      vec![TokenType::Character,
                           TokenType::Character,
                           TokenType::Character,
                           TokenType::Character,
                           TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '\0', ' ', 'a', '\0']);
  }

  #[test]
  fn scan_ascii_hex_escapes() {
    let mut s = Scanner::new(&String::from("\\x61\\x0a\\x00a"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '\n', '\0', 'a', '\0']);
  }

  #[test]
  fn scan_invalid_ascii_hex_escape() {
    let mut s = Scanner::new(&String::from("\\xj"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['\0', 'j', '\0']);
  }

  fn scan_escaped_reserved_characters() {
    let mut s = Scanner::new(&String::from("\\|\\*\\(\\)\\[\\]\\^\\?\\+"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character,
                          TokenType::Character]);
    test_token_images(&tokens,
                      vec!['|', '*', '(', ')', '[', ']', '^', '?', '+', '\0']);
  }

  #[test]
  fn scan_union() {
    let mut s = Scanner::new(&String::from("a|"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Union,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '|', '\0']);
  }

  #[test]
  fn scan_star() {
    let mut s = Scanner::new(&String::from("a*"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Star,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '*', '\0']);
  }

  #[test]
  fn scan_parens() {
    let mut s = Scanner::new(&String::from("(a)"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::LParen,
                          TokenType::Character,
                          TokenType::RParen,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['(', 'a', ')', '\0']);
  }

  #[test]
  fn scan_brackets_with_caret() {
    let mut s = Scanner::new(&String::from("[^a]"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::LBracket,
                          TokenType::Caret,
                          TokenType::Character,
                          TokenType::RBracket,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['[', '^', 'a', ']', '\0']);
  }

  #[test]
  fn scan_question() {
    let mut s = Scanner::new(&String::from("a?"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Question,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '?', '\0']);
  }

  #[test]
  fn scan_plus() {
    let mut s = Scanner::new(&String::from("a+"));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Character,
                          TokenType::Plus,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['a', '+', '\0']);
  }

  #[test]
  fn scan_wildcard() {
    let mut s = Scanner::new(&String::from("."));
    let tokens = scan_all(&mut s);
    test_token_types(&tokens,
                     vec![TokenType::Range,
                          TokenType::EOF]);
    test_token_images(&tokens,
                      vec!['.', '\0']);
  }
}
