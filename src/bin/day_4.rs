use std::{fs, ops::Sub};

fn main() {
  let binding = fs::read_to_string("input/day_4.txt").expect("mlem");
  let lines_vec = &binding
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let mut counter = 0;
  for (line_index, line) in lines_vec.iter().enumerate() {
    for (char_index, char) in line.iter().enumerate() {
      if *char == 'X' {
        let ms = get_surrounding_letter(line_index, char_index, lines_vec, 'M');
        if ms.len() != 0 {
          for m in ms {
            let next_a = get_next_letter(line_index, char_index, lines_vec, m, 'A');
            if next_a.len() != 0 {
              for a in next_a {
                let next_s = get_next_letter(m.1, m.0, lines_vec, a, 'S');
                counter += next_s.len();
              }
            }
          }
        }
      }
    }
  }
  dbg!(counter);

  // Part 2
  let mut counter_2 = 0;
  for (line_index, line) in lines_vec.iter().enumerate() {
    for (char_index, char) in line.iter().enumerate() {
      if *char == 'A' {
        let corners = get_surrounding_corners(line_index, char_index, lines_vec);
        if corners.len() == 4 {
          if corners == ['M', 'M', 'S', 'S']
            || corners == ['M', 'S', 'M', 'S']
            || corners == ['S', 'S', 'M', 'M']
            || corners == ['S', 'M', 'S', 'M']
          {
            counter_2 += 1;
          }
        }
      }
    }
  }
  dbg!(counter_2);
}

fn get_surrounding_letter(
  line_index: usize,
  char_index: usize,
  lines: &Vec<Vec<char>>,
  wanted_letter: char,
) -> Vec<(usize, usize, char)> {
  let surrounding_indexes = (-1..2).flat_map(|i: isize| (-1..2).map(move |j| (i, j)));
  let surrounding_letters = surrounding_indexes
    .filter_map(|(x, y)| {
      let x_index: usize = ((char_index as isize) + x).try_into().ok()?;
      let y_index: usize = ((line_index as isize) + y).try_into().ok()?;
      let letter = lines.get(y_index)?.get(x_index)?;
      if *letter == wanted_letter {
        Some((x_index, y_index, *letter))
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  surrounding_letters
}

fn get_next_letter(
  line_index: usize,
  char_index: usize,
  lines: &Vec<Vec<char>>,
  prev_letter: (usize, usize, char),
  next_letter: char,
) -> Vec<(usize, usize, char)> {
  let (prev_x_index, prev_y_index, _) = prev_letter;
  let x_direction = (prev_x_index as isize).sub(char_index as isize);
  let y_direction = (prev_y_index as isize).sub(line_index as isize);
  let possible_char_x = prev_x_index as isize + x_direction;
  let possible_char_y = prev_y_index as isize + y_direction;

  let surrounding_as = get_surrounding_letter(prev_y_index, prev_x_index, lines, next_letter);
  let continuing_as = surrounding_as
    .iter()
    .filter_map(|(next_x, next_y, letter)| {
      if *next_x == possible_char_x as usize && *next_y == possible_char_y as usize {
        Some((*next_x, *next_y, *letter))
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  continuing_as
}

fn get_surrounding_corners(
  line_index: usize,
  char_index: usize,
  lines: &Vec<Vec<char>>
) -> Vec<char> {
  let surrounding_letters = [(-1, 1), (1, 1), (-1, -1), (1, -1)]
    .iter()
    .filter_map(|(x, y)| {
      let x_index: usize = ((char_index as isize) + x).try_into().ok()?;
      let y_index: usize = ((line_index as isize) + y).try_into().ok()?;
      let line = lines.get(y_index)?;
      Some(*line.get(x_index)?)
    })
    .collect::<Vec<_>>();
  surrounding_letters
}
