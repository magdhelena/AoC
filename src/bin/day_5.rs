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
      if line.iter().enumerate().any(|(i, page)| {
        let empty_vec = &vec![];
        let cant_be_before_page = rules_grouping.get(page).unwrap_or(empty_vec);
        *(&line[..i]
          .iter()
          .any(|other_page| cant_be_before_page.contains(other_page)))
      }) {
        None
      } else {
        Some(line[(line.len() - 1) / 2])
      }
    })
    .sum();
  dbg!(result_1);

  // Part 2
  let result_2: usize = updates
    .iter()
    .filter(|line| {
      line.iter().enumerate().any(|(i, page)| {
        let empty_vec = &vec![];
        let cant_be_before_page = rules_grouping.get(page).unwrap_or(empty_vec);
        *(&line[..i]
          .iter()
          .any(|other_page| cant_be_before_page.contains(other_page)))
      })
    })
    .map(|wrong_update| {
      let mut new_update: Vec<usize> = vec![];
      for &page in wrong_update.iter() {
        let empty_vec = &vec![];
        let cant_be_before_page = rules_grouping.get(&page).unwrap_or(empty_vec);
        insert_into_sorted_spot(&mut new_update, page, cant_be_before_page);
      }
      new_update[(new_update.len() - 1) / 2]
    })
    .sum();
  dbg!(result_2);
}

fn insert_into_sorted_spot(
  new_update: &mut Vec<usize>,
  page: usize,
  cant_be_before_page: &Vec<usize>,
) {
  match new_update
    .iter()
    .position(|new_page| cant_be_before_page.contains(new_page))
  {
    Some(i) => new_update.insert(i, page),
    None => new_update.push(page),
  }
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
