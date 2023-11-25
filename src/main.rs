mod regex;
mod parser;

fn main() {
  println!("rex - tiny regular expression engine\n");

  // let test_cases = vec!["abc", "a", "abcde", "an"];

  // for t in test_cases {
      // let re = regex::RegEx::new("abc");
      // println!("Match '{}'? => {}", t, re.eval(String::from(t)).success);
  // }

  let re = regex::RegEx::new("a(a|b)*");
  let test_cases = vec!["aba", "no", "bababa", "abababa", "", "axxbxaba",
                        "aaaab"
                        ];

  for t in test_cases {
    println!("Match '{}'? => {}", t, re.eval(String::from(t)).success);
  }
}
