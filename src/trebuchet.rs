use std::fs;

const CHARS: [(&str, u32); 19] = [
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9),
  ("1", 1),
  ("2", 2),
  ("3", 3),
  ("4", 4),
  ("5", 5),
  ("6", 6),
  ("7", 7),
  ("8", 8),
  ("9", 9),
  ("0", 0),
];

fn trebuchet(lines: &str) -> (u32, u32) {
  let ans_1 = lines.lines().flat_map(get_line_val_1).sum::<u32>();
  let ans_2 = lines.lines().flat_map(get_line_val_2).sum::<u32>();

  (ans_1, ans_2)
}

fn get_line_val_1(line: &str) -> Option<u32> {
  let pattern: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let c1 = line.find(&pattern);
  let c2 = line.rfind(&pattern);
  Some((line.chars().nth(c1?)? as u32 - 0x30) * 10u32 + (line.chars().nth(c2?)? as u32 - 0x30))
}

fn get_line_val_2(line: &str) -> Option<u32> {
  // (index in line, value)
  let c1: u32 = CHARS
    .iter()
    .flat_map(|(word, value)| line.find(word).map(|i| (i, value)))
    .min_by_key(|(i, &value)| *i)
    .map(|(_i, value)| *value)?;
  let c2: u32 = CHARS
    .iter()
    .flat_map(|(word, value)| line.rfind(word).map(|i| (i, value)))
    .max_by_key(|(i, &value)| *i)
    .map(|(_i, value)| *value)?;

  Some(c1 * 10u32 + c2)
}

pub fn solve(input: String) {
  println!("With solutions {:?}", trebuchet(&input));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sample_1_trebuchet() {
    let input =
      fs::read_to_string("inputs/trebuchet_1_sample.txt").expect("Could not read input file");

    let (ans_1, _) = trebuchet(&input);
    assert_eq!(ans_1, 142);
  }

  #[test]
  fn sample_2_trebuchet() {
    let input =
      fs::read_to_string("inputs/trebuchet_2_sample.txt").expect("Could not read input file");

    let (_, ans_2) = trebuchet(&input);
    assert_eq!(ans_2, 281);
  }
}
