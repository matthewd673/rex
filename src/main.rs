mod regex;
mod parser;

fn main() {
  println!("rex - tiny regular expression engine\n");

  let re = regex::RegEx::new("(ab|a)*");
  let test_cases = vec!["aba", "no", "bababa", "abababa", "", "axxbxaba",
                        "aaaab"
                        ];
  // for t in test_cases {
    // println!("Match '{}'? => {}", t, re.eval(t));
  // }

  // compiler tests
  // working
  // regex::RegEx::new("/(|abc)*xyz*/");
  // // aren't working but should
  // regex::RegEx::new("/|/");
  // regex::RegEx::new("/()*/");
}
