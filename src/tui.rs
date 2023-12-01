pub struct Color {
  // Empty
}

impl Color {
  pub const BLACK: &str = "30";
  pub const RED: &str = "31";
  pub const GREEN: &str = "32";
  pub const YELLOW: &str = "33";
  pub const BLUE: &str = "34";
  pub const MAGENTA: &str = "35";
  pub const CYAN: &str = "36";
  pub const WHITE: &str = "37";
  pub const DEFAULT: &str = "39";
}

pub struct TextStyle<'a> {
  pub foreground: &'a str,
  // pub background: &'a str,
  pub bold: bool,
}

impl TextStyle<'static> {
  pub fn apply(&self) {
    print!("\x1b[");

    // make bold (if applicable)
    if self.bold {
      print!("1;");
    }

    // set colors
    print!("{}", self.foreground);
    // print!(";");

    // print!("{}", self.background);
    print!("m");
  }
}
