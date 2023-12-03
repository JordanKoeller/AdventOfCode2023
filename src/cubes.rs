struct Game {
  ind: u32,
  rounds: Vec<Round>,
}

impl Game {
  fn parse(description: &str) -> Self {
    let game_rounds: Vec<&str> = description.split(": ").collect();

    Self {
      ind: game_rounds[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
      rounds: game_rounds[1].split("; ").map(Round::parse).collect(),
    }
  }
}

struct Round {
  red: u32,
  green: u32,
  blue: u32,
}

impl Round {
  pub fn parse(description: &str) -> Self {
    let mut red = 0u32;
    let mut green = 0u32;
    let mut blue = 0u32;

    for elem in description.split(", ") {
      let num_name: Vec<&str> = elem.split(" ").collect();
      match num_name[1] {
        "red" => {
          red = num_name[0].parse().unwrap();
        }
        "green" => {
          green = num_name[0].parse().unwrap();
        }
        "blue" => {
          blue = num_name[0].parse().unwrap();
        }
        _ => {}
      }
    }

    Self { red, green, blue }
  }
}

fn cubes_1(games_desc: &Vec<String>, red: u32, green: u32, blue: u32) -> u32 {
  let games: Vec<Game> = games_desc.iter().map(|s| Game::parse(s)).collect();

  games
    .iter()
    .filter(|game| {
      let max_red = game.rounds.iter().map(|round| round.red).max().unwrap();
      let max_green = game.rounds.iter().map(|round| round.green).max().unwrap();
      let max_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();

      if max_red <= red && max_green <= green && max_blue <= blue {
        true
      } else {
        false
      }
    })
    .map(|game| game.ind)
    .sum::<u32>()
}

fn cubes_2(games_desc: &Vec<String>) -> u64 {
  let games: Vec<Game> = games_desc.iter().map(|s| Game::parse(s)).collect();

  games
    .iter()
    .map(|game| {
      let max_red = game.rounds.iter().map(|round| round.red).max().unwrap();
      let max_green = game.rounds.iter().map(|round| round.green).max().unwrap();
      let max_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();

      let ret = (max_red * max_green * max_blue) as u64;

      // println!("RGB = {} {} {} => {}", max_red, max_green, max_blue, ret);
      ret
    })
    .sum::<u64>()
}

pub fn solve(input: String) -> (u32, u64) {
  let descs: Vec<String> = input.lines().map(|s| s.to_string()).collect();

  (cubes_1(&descs, 12, 13, 14), cubes_2(&descs))
}

#[cfg(test)]
mod tests {
  use std::fs;

  use super::*;

  #[test]
  fn cubes_sample_1() {
    let input = fs::read_to_string("inputs/cubes_1_sample.txt").expect("Could not read input file");

    let descs: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let ans = cubes_1(&descs, 12, 13, 14);
    assert_eq!(ans, 8);

    let ans_2 = cubes_2(&descs);
    assert_eq!(ans_2, 2286);
  }
}
