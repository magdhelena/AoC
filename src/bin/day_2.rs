use std::{fs, isize};

fn main() {
  let binding = &fs::read_to_string("./day_2.txt").expect("mlem");
  let reports = binding.lines().map(parse_line);
  let acc: usize = reports
    .filter(|report| {
      let (diff_bools, sign_bools): (Vec<_>, Vec<_>) =
        report.iter().zip(&report[1..]).map(parse_values).unzip();
      is_report_safe(diff_bools, sign_bools)
    })
    .count();
  dbg!(acc);
}

fn parse_line(line: &str) -> Vec<isize> {
  line
    .split_whitespace()
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>()
}

fn parse_values((first, second): (&isize, &isize)) -> (bool, bool) {
  let diff = second - first;
  let safe_diff = diff.abs() > 0 && diff.abs() <= 3;
  (safe_diff, diff.is_positive())
}

fn is_report_safe(diff_bools: Vec<bool>, sign_bools: Vec<bool>) -> bool {
  diff_bools.iter().all(|value| *value) && sign_bools.iter().all(|value| *value == sign_bools[0])
}
