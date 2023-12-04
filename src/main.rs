mod regex;
mod scanner;
mod parser;
mod tui;

use std::env;
use std::fs;
use std::path::Path;
use std::collections::VecDeque;

struct ExecOptions {
  no_groups: bool,  // don't print matching groups
}

impl ExecOptions {
  fn new() -> Self {
    return ExecOptions {
      no_groups: false,
    };
  }
}

fn execute_headless(filename: String, expr: String, options: &ExecOptions) {
  // load file from args
  // TODO: better handling for load failed
  let file_text = fs::read_to_string(&filename)
                      .expect("Failed to load file");
  let file_lines = file_text.split("\n");

  let re = regex::RegEx::new(&expr);
  for l in file_lines {
    let match_data = re.match_all(String::from(l));
    for m in match_data {
      println!("{}", m.groups[0].string);
      if !options.no_groups {
        for g in 1..m.groups.len() {
          println!("  {}: {}", g, m.groups[g].string);
        }
      }
    }
  }
}

fn execute_interactive(filename: String, options: &ExecOptions) {
  println!("TODO: interactive");
}

fn parse_args() {
  let mut args: VecDeque<String> = env::args().collect();

  let mut expr: Option<String> = None;
  let mut filename: Option<String> = None;
  let mut options = ExecOptions::new();

  args.pop_front(); // skip first arg (executable path)

  while args.len() > 0 {
    let a = args.pop_front().unwrap();

    // check for flags
    if &a == "-ng" || &a == "--no-groups" {
      options.no_groups = true;
    }

    // if no filename, try to find one
    if matches!(filename, None) {
      let potential_path = Path::new(&a);
      // is a filename set it
      if potential_path.is_file() {
        filename = Some(a);
        continue;
      }
    }

    // otherwise, assume its an expression
    if matches!(expr, None) {
      expr = Some(a);
      continue;
    }
  }

  // choose mode based on provided args
  match (filename, expr) {
    (Some(f), Some(e)) => {
      execute_headless(f, e, &options);
    },
    (Some(f), None) => {
      execute_interactive(f, &options);
    },
    _ => {
      println!("invalid argument set");
    }
  }
}

fn main() {
  parse_args();
}
