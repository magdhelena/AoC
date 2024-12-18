use std::{collections::HashMap, fs};

fn main() {
  let rules_binding = fs::read_to_string("input/day_5_rules.txt").expect("mlem");
  let rules_tuples: Vec<(usize, usize)> = rules_binding
    .lines()
    .map(parse_rules_line)
    .collect::<Vec<_>>();

  let mut rules_grouping: HashMap<usize, Vec<usize>> = HashMap::new();
  for (num1, num2) in rules_tuples {
    rules_grouping
      .entry(num1)
      .and_modify(|rule_vec| rule_vec.push(num2))
      .or_insert(vec![num2]);
  }

  let updates_binding = fs::read_to_string("input/day_5_updates.txt").expect("mlem");
  let updates = updates_binding
    .lines()
    .map(parse_updates_line)
    .collect::<Vec<_>>();

  let result_1: usize = updates
    .iter()
    .filter_map(|line| {
      let mut middle_page = Some(line[(line.len() - 1) / 2]);
      for (i, page) in line.iter().enumerate() {
        let cant_be_before_page = match rules_grouping.get(page) {
          Some(x) => x,
          None => {
            continue;
          }
        };
        if *(&line[..i]
          .iter()
          .any(|other_page| cant_be_before_page.contains(other_page)))
        {
          middle_page = None
        }
      }
      middle_page
    })
    .sum();
  dbg!(result_1);

  // Part 2
  let result_2: usize = updates
    .iter()
    .filter(|line| {
      for (i, page) in line.iter().enumerate() {
        let cant_be_before_page = match rules_grouping.get(page) {
          Some(x) => x,
          None => {
            continue;
          }
        };
        if *(&line[..i]
          .iter()
          .any(|other_page| cant_be_before_page.contains(other_page)))
        {
          return true;
        }
      }
      false
    })
    .map(|wrong_update| {
      let mut new_update: Vec<usize> = vec![];
      for page in wrong_update.iter() {
        let cant_be_before_page = match rules_grouping.get(page) {
          Some(x) => x,
          None => {
            new_update.push(*page);
            continue;
          }
        };

        if new_update.len() == 0 {
          new_update.push(*page);
        } else {
          for (i, new_page) in new_update.iter().enumerate() {
            if cant_be_before_page.contains(new_page) {
              new_update.insert(i, *page);
              break;
            } else if i + 1 == new_update.len() {
              new_update.push(*page);
              break;
            }
          }
        }
      }
      new_update[(new_update.len() - 1) / 2]
    })
    .sum();
  dbg!(result_2);
}

fn parse_rules_line(line: &str) -> (usize, usize) {
  let parsed_line: [_; 2] = line
    .split('|')
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
  parsed_line.into()
}

fn parse_updates_line(line: &str) -> Vec<usize> {
  line
    .split(',')
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>()
}
