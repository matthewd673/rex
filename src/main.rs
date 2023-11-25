mod regex;
mod parser;

fn main() {
  println!("rex - tiny regular expression engine\n");

  let re = regex::RegEx::new("a(a|b)*xyz*");
  let test_cases = vec!["abaxy", "no", "abababaxyzzzzzzz", "abababaxy", "",
                        "axxbxaba", "aaaabxyzz", "axy",
                        ];

  for t in test_cases {
    println!("Match '{}'? => {}", t, re.eval(String::from(t)).success);
  }
}
