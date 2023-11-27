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

pub struct CharRange {
  pub min: char,
  pub max: char,
}

impl CharRange {
  fn new(min: char, max: char) -> Self {
    return CharRange { min, max };
  }
}

pub struct Token {
  pub t_type: TokenType,
  pub image: char,
  pub has_range: bool,
  pub range: CharRange,
}

impl Token {
  fn new(t_type: TokenType, image: char) -> Self {
    return Token {
      t_type,
      image,
      has_range: false,
      range: CharRange::new('\0', '\0'),
    };
  }
}

struct Scanner {
  // input: String,
  chars: Vec<char>,
  index: usize,
}

fn char_to_hex(h: char) -> u32 {
  match h {
    '0' => 0x0, '1' => 0x1, '2' => 0x2, '3' => 0x3, '4' => 0x4, '5' => 0x5,
    '6' => 0x6, '7' => 0x7, '8' => 0x8, '9' => 0x9, 'a' => 0xa, 'b' => 0xb,
    'c' => 0xc, 'd' => 0xd, 'e' => 0xe, 'f' => 0xf, _ => 0x0,
  }
}

impl Scanner {
  fn new(input: &String) -> Self {
    let chars = input.chars().collect();
    return Scanner {
      chars,
      index: 0usize,
    };
  }

  fn scan_next(&mut self) -> Token {
    let t = match self.chars.get(self.index) {
      Some(c) => self.char_to_token(*c),
      None => Token::new(TokenType::EOF, '\0'),
    };

    self.index += 1;

    return t;
  }

  fn handle_escape(&mut self) -> Token {
    let mut escape_len = 0;
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
          'u' => {
            escape_len += 1;
          }
          't' => { return Token::new(TokenType::Character, '\t'); },
          'n' => { return Token::new(TokenType::Character, '\n'); },
          'r' => { return Token::new(TokenType::Character, '\r'); },
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
    let min_char: char = char::from_u32(0x0000).unwrap();
    let max_char: char = char::from_u32(0xFFFF).unwrap();

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
        has_range: true,
        range: CharRange::new(min_char, max_char),
      },
      '\\' => self.handle_escape(),
      _ => Token::new(TokenType::Character, c),
    }
  }
}

#[derive(Debug)]
pub enum NodeType {
  Error,
  Empty,

  Word,
  Charset,
  Union,
  Star,
  Group,
  MatchGroup,
}

pub struct Bounds {
  pub min: u32,
  pub max: u32,
}

pub struct TreeNode {
  pub n_type: NodeType,
  pub children: Vec<TreeNode>,
  pub image: Vec<char>,     // used by Words
  pub repeats: Bounds,      // used by Star-likes (?, etc.)
  pub negated: bool,        // used by Charsets
  pub ranges: Vec<CharRange>,  // used by Charsets
}

impl TreeNode {
  fn new(n_type: NodeType) -> Self {
    return TreeNode {
      n_type,
      children: vec![],
      image: vec![],
      repeats: Bounds { min: 0, max: 0 },
      negated: false,
      ranges: vec![],
    };
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
  pub fn new(input: &String) -> Self {
    let scanner = Scanner::new(input);

    return Parser {
      scanner,
      next_token: Token::new(TokenType::Error, '\0'),
    };
  }

  pub fn parse(&mut self) -> TreeNode {
    // point to first character
    self.next_token = self.scanner.scan_next();
    // parse
    let tree = self.parse_root();
    print_node(&tree, 0);
    return tree;
  }

  fn eat(&mut self, expected: TokenType) {
    let t = &self.next_token;

    if !matches!(&t.t_type, expected) { // TODO: why is this a warning?
      println!("syntax error: expected {:?}, saw {:?}",
               t.t_type,
               expected
               );
    }
    else {
      self.next_token = self.scanner.scan_next();
    }
  }

  fn parse_root(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // total -> expr eof
      TokenType::Character | TokenType::Range |
      TokenType::LBracket | TokenType::LParen |
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
      TokenType::Character | TokenType::Range |
      TokenType::LBracket | TokenType::LParen |
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
      TokenType::Character | TokenType::Range |
      TokenType::LBracket | TokenType::LParen => {
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
        // create word node
        let mut word_node = TreeNode::new(NodeType::Word);
        word_node.image.push(self.next_token.image);

        // continue parsing
        self.eat(TokenType::Character);

        return word_node;
      },
      // atom -> range
      TokenType::Range => {
        // create charset node
        let mut charset_node = TreeNode::new(NodeType::Charset);
        charset_node.ranges.push(
          CharRange::new(self.next_token.range.min, self.next_token.range.max)
        );

        // continue parsing
        self.eat(TokenType::Range);

        return charset_node;
      },
      // atom -> ( expr )
      TokenType::LParen => {
        // println!("atom -> ( expr )");
        self.eat(TokenType::LParen);
        let expr_node = self.parse_expr();
        self.eat(TokenType::RParen);

        return TreeNode::make_group(expr_node, NodeType::MatchGroup);
      },
      // atom -> [ neg charset ]
      TokenType::LBracket => {
        // println!("atom -> [ neg charset ]");
        self.eat(TokenType::LBracket);
        let neg_node = self.parse_neg();
        self.eat(TokenType::RBracket);

        return neg_node;
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
      // star -> ?
      TokenType::Question => {
        // create star node with repeat count
        let mut star_node = TreeNode::new(NodeType::Star);
        star_node.add_child(lhs);
        star_node.repeats.min = 0;
        star_node.repeats.max = 1;

        // continue parsing
        self.eat(TokenType::Question);

        return star_node;
      },
      // star -> +
      TokenType::Plus => {
        // create star node with repeat count
        let mut star_node = TreeNode::new(NodeType::Star);
        star_node.add_child(lhs);
        star_node.repeats.min = 1;
        star_node.repeats.max = 0; // no maximum

        // continue parsing
        self.eat(TokenType::Plus);

        return star_node;
      },
      // star -> ε
      TokenType::Character | TokenType::Range |
      TokenType::LBracket | TokenType::LParen |
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
      TokenType::Character | TokenType::Range |
      TokenType::LBracket | TokenType::LParen |
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

  fn parse_neg(&mut self) -> TreeNode {
    match self.next_token.t_type {
      // neg -> ^ charset
      TokenType::Caret => {
        self.eat(TokenType::Caret);

        return self.parse_charset(true);
      },
      // neg -> charset
      TokenType::Character => {
        return self.parse_charset(false);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing neg",
                 self.next_token.t_type);
        return TreeNode::new(NodeType::Error);
      },
    }
  }

  fn parse_charset(&mut self, negated: bool) -> TreeNode {
    match self.next_token.t_type {
      // charset -> character charset
      TokenType::Character => {
        // create new charset node
        let mut charset_node = TreeNode::new(NodeType::Charset);
        charset_node.negated = negated;

        // parse as many characters as possible
        while matches!(self.next_token.t_type, TokenType::Character) {
          // get next character
          let c = self.next_token.image;
          self.eat(TokenType::Character);

          // turn two characters separated by a '-' into a range
          if charset_node.ranges.len() > 1 &&
             charset_node.ranges.last().unwrap().min == '-' {
             // remove the dash
             charset_node.ranges.pop();
             // modify the previous char into a range
             let mut prev_range = charset_node.ranges.pop().unwrap();
             prev_range.max = c;
             charset_node.ranges.push(prev_range);
          }
          // just add a character to the ranges
          else {
            charset_node.ranges.push(CharRange::new(c, c));
          }
        }

        return charset_node;
      },
      // charset -> ε
      TokenType::RBracket => {
        // epsilon case cannot be negated (because that makes no sense)
        // NOTE: this is not how PCRE2 does it: `[^]` is interpreted as
        //   "not rbracket" and throws a syntax error for missing rbracket
        return TreeNode::new(NodeType::Charset);
      },
      _ => {
        println!("syntax error: saw {:?} while parsing charset",
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
  print!("]");

  if node.negated {
    print!(" negated");
  }

  println!("");

  for n in &node.children {
    print_node(n, depth + 1);
  }
}
