mod nfa;
mod regex;
mod compiler;

fn main() {
  println!("rex - tiny regular expression engine\n");

  let re = regex::RegEx::new("(ab|a)*");
  let test_cases = vec!["aba", "no", "bababa", "abababa", "", "axxbxaba",
                        "aaaab"
                        ];
  for t in test_cases {
    println!("Match '{}'? => {}", t, re.eval(t));
  }

  // let tokens = compiler::scan("(ab|a)*");
  // for t in tokens {
    // println!("{:?}: '{}'", t.t_type, t.image);
  // }

  let mut compiler = compiler::Compiler::new(String::from("/(ab|a)*/"));
  compiler.compile();
}
