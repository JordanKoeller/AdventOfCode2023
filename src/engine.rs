fn engine(grid_desc: String) -> (u32, u32) {
  let ignore_chars: Vec<char> = "0123456789.".chars().collect();

  let mut grid: Vec<Vec<char>> = grid_desc.lines().map(|c| c.chars().collect()).collect();

  let w = grid[0].len();
  let h = grid.len();

  let mut ret = 0;
  let mut gears = 0;

  for x in 0..w {
    for y in 0..h {
      if !ignore_chars.contains(&(grid[y][x] as char)) {
        let nums = process_symbol(&mut grid, x as i32, y as i32);
        nums.iter().for_each(|n| ret += n);
        if grid[y][x] == '*' && nums.len() == 2 {
          gears += nums[0] * nums[1];
        }
      }
    }
  }

  (ret, gears)
}

fn process_symbol(grid: &mut Vec<Vec<char>>, x: i32, y: i32) -> Vec<u32> {
  let mut sum: Vec<u32> = Vec::new();
  for dx in [-1i32, 0i32, 1i32] {
    for dy in [-1i32, 0i32, 1i32] {
      let i: i32 = x + dx;
      let j: i32 = y + dy;

      if let Some(c) = grid.get(j as usize).and_then(|v| v.get(i as usize)) {
        if is_num(*c) {
          sum.push(parse_num(grid, i as usize, j as usize));
        }
      }
    }
  }
  sum
}

fn parse_num(grid: &mut Vec<Vec<char>>, mut i: usize, j: usize) -> u32 {
  let mut ret = 0;

  // We may have started in the middle of a number. So scan left until
  // there are no more numeric characters.
  while i > 0 && is_num(grid[j][i]) {
    i -= 1;
  }
  // Correct if over-scanned.
  if !is_num(grid[j][i]) {
    i += 1
  }

  // Iterate through chars, accumulate ret.
  while i < grid[j].len() {
    if is_num(grid[j][i]) {
      // Get value, then remove so not double-counted
      ret = ret * 10 + grid[j][i] as u32 - 0x30;
      grid[j][i] = '.';
    } else {
      break;
    }
    i += 1;
  }

  ret
}

fn is_num(c: char) -> bool {
  "0123456789".contains(c)
}

pub fn solve(input: String) -> (u32, u32) {
  engine(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn engine_sample_1() {
    let input =
      fs::read_to_string("inputs/engine_1_sample.txt").expect("Could not read input file");

    let (ans, gears) = engine(input);

    assert_eq!(ans, 4361);
    assert_eq!(gears, 467835);
  }
}
