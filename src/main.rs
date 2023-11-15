mod regex;

fn main() {
  println!("rex - tiny regular expression engine\n");

  let re = regex::RegEx::new("(ab|a)*");

  let test_cases = vec!["aba", "no", "bababa", "abababa", "", "axxbxaba",
                        "aaaab"
                        ];

  for t in test_cases {
    println!("Match '{}'? => {}", t, re.eval(t));
  }

}
