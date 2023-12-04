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

  fn max_vals(&self) -> (u32, u32, u32) {
    let max_red = self.rounds.iter().map(|round| round.red).max().unwrap();
    let max_green = self.rounds.iter().map(|round| round.green).max().unwrap();
    let max_blue = self.rounds.iter().map(|round| round.blue).max().unwrap();

    (max_red, max_green, max_blue)
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

fn cubes_1(games: &Vec<Game>, red: u32, green: u32, blue: u32) -> u32 {
  games
    .iter()
    .filter(|game| {
      let (max_red, max_green, max_blue) = game.max_vals();

      max_red <= red && max_green <= green && max_blue <= blue
    })
    .map(|game| game.ind)
    .sum::<u32>()
}

fn cubes_2(games: &Vec<Game>) -> u32 {
  games
    .iter()
    .map(|game| {
      let (max_red, max_green, max_blue) = game.max_vals();

      max_red * max_green * max_blue
    })
    .sum::<u32>()
}

pub fn solve(input: String) -> (u32, u32) {
  let descs: Vec<String> = input.lines().map(|s| s.to_string()).collect();
  let games: Vec<Game> = descs.iter().map(|s| Game::parse(s)).collect();

  (cubes_1(&games, 12, 13, 14), cubes_2(&games))
}

#[cfg(test)]
mod tests {
  use crate::testing::test_with_input;
  use super::*;

  #[test]
  fn cubes_sample_1() {
    test_with_input("cubes", solve, 8, 2286);
  }
}
