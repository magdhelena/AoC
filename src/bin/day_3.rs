use std::fs;

use regex::Regex;

fn main() {
  let binding = fs::read_to_string("input/day_3.txt").expect("mlem");
  let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  let matches = mul_re.captures_iter(&binding);
  let total: usize = matches.map(|mul| {
    let (_, [num1, num2]): (&str, [&str; 2]) = mul.extract();
    num1.parse::<usize>().unwrap()*num2.parse::<usize>().unwrap()
  }).sum();
  dbg!(total);
}
