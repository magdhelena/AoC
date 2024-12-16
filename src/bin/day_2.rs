use std::{fs, isize};

fn main() {
  let binding = &fs::read_to_string("input/day_2.txt").expect("mlem");
  let reports = binding.lines().map(parse_line).collect::<Vec<Vec<isize>>>();
  let mut salvageable_reports = 0;
  let acc = reports
    .iter()
    .filter(|report| {
      let report_safety = get_report_safety(report);
      if !report_safety {
        let safety = retry_report(report);
        if safety {
          salvageable_reports += 1;
        }
      }
      report_safety
    })
    .count();
  dbg!(acc);
  dbg!(salvageable_reports);
  dbg!(acc + salvageable_reports);
}

fn parse_line(line: &str) -> Vec<isize> {
  line
    .split_whitespace()
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>()
}

fn get_report_safety(report: &&Vec<isize>) -> bool {
  let (diff_bools, sign_bools): (Vec<_>, Vec<_>) =
    report.iter().zip(&report[1..]).map(parse_values).unzip();
  let report_safety = is_report_safe(&diff_bools, &sign_bools);
  report_safety
}

fn parse_values((first, second): (&isize, &isize)) -> (bool, bool) {
  let diff = second - first;
  let safe_diff = diff.abs() > 0 && diff.abs() <= 3;
  (safe_diff, diff.is_positive())
}

fn retry_report(report: &&Vec<isize>) -> bool {
  let mut safe = false;
  for (i, _) in report.iter().enumerate() {
    let mut new_report = (*report).clone();
    new_report.remove(i);
    let is_safe = get_report_safety(&&new_report);
    if is_safe {
      safe = true;
      break;
    }
  }
  safe
}

fn is_report_safe(diff_bools: &Vec<bool>, sign_bools: &Vec<bool>) -> bool {
  diff_bools.iter().all(|value| *value) && sign_bools.iter().all(|value| *value == sign_bools[0])
}
