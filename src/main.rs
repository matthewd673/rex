mod regex;
mod parser;

use std::env;
use std::fs;

fn main() {
  println!("rex - tiny regular expression engine\n");

  // parse args
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("args: <filename>");
    return;
  }

  // load file from args
  let file_text = fs::read_to_string(&args[1])
                    .expect("Failed to load file");
  let file_lines = file_text.split("\n");

  // let re = regex::RegEx::new("a(a|b)*xyz*");
  // let test_cases = vec!["abaxy", "no", "abababaxyzzzzzzz", "abababaxy", "",
                        // "axxbxaba", "aaaabxyzz", "axy",
                        // ];

  let re = regex::RegEx::new("(a|\\()*b*");
  println!("re = {}", re.expr);
  for l in file_lines {
    println!("Match '{}'? => {}", l, re.eval(String::from(l)).success);
  }
}
