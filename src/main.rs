mod regex;
mod parser;

use std::env;
use std::fs;

fn execute_headless(expr: String, filename: String) {
  println!("expr: {}", expr);
  println!("filename: {}", filename);
  // load file from args
  // TODO: better handling for load failed
  let file_text = fs::read_to_string(&filename)
                    .expect("Failed to load file");
  let file_lines = file_text.split("\n");

  let re = regex::RegEx::new("[a-zA-Z]+({[0-9]}+)?[^0-9]?");
  println!("re = {}", re.expr);
  for l in file_lines {
    let match_data = re.match_all(String::from(l));
    for m in match_data {
      println!("{}", m.groups[0].string);
      for g in 1..m.groups.len() {
        println!("  {}: {}", g, m.groups[g].string);
      }
    }
  }
}

fn main() {
  println!("rex - tiny regular expression engine\n");

  // parse args
  let mut args: Vec<String> = env::args().collect();
  // no args given, open interactive
  if args.len() < 1 {
    println!("TODO: interactive");
  }
  // if one arg, then thats the filename
  // open it in interactive
  else if args.len() == 1 {
    println!("TODO: interactive");
  }
  // if two or more args then first two are regex and filename
  else if args.len() > 1 {
    execute_headless(args.pop().unwrap(), args.pop().unwrap());
  }
}
