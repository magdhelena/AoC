use std::{collections::HashMap, fs};

fn main() {
  let binding = fs::read_to_string("./day_1.txt").expect("mlem");
  let (mut first, mut second): (Vec<_>, Vec<_>) = binding.lines().map(parse_line).unzip();
  let _ = &first.sort();
  let _ = &second.sort();
  let acc: isize = first.iter().zip(&second).map(|(a, b)| (a-b).abs()).sum();
  dbg!(acc);

  //Part 2
  let mut counter: HashMap<isize, isize> = HashMap::new();
  for num in second {
    counter.entry(num).and_modify(|x| *x += 1).or_insert(1);
  }
  let acc_2_electric_boogaloo: isize = first.into_iter().fold(0, |acc, num| acc + counter.get(&num).unwrap_or(&0)*num);
  dbg!(acc_2_electric_boogaloo);
}

fn parse_line(line: &str) -> (isize, isize) {
  let parsed_line: [_; 2] = line
    .split_whitespace()
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
  parsed_line.into()
}
