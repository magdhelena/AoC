use std::fs;

use regex::Regex;

fn main() {
  let binding = fs::read_to_string("input/day_3.txt").expect("mlem");
  let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
  let matches = mul_re.captures_iter(&binding);

  let mut flag = true;
  let total: usize = matches
    .map(|mul| {
      let num1 = mul.get(1).map_or("", |m| m.as_str());
      let num2 = mul.get(2).map_or("", |m| m.as_str());
      let do_word = mul.get(3).is_some();
      let dont_word = mul.get(4).is_some();

      if do_word {
        flag = true
      };
      if dont_word {
        flag = false
      };

      if flag && !num1.is_empty() && !num2.is_empty() {
        num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap()
      } else {
        0
      }
    })
    .sum();
  dbg!(total);
}
