struct Card {
  wins: Vec<u32>,
  held: Vec<u32>,
}

impl Card {
  fn parse(line: &str) -> Self {
    let mut card_nums = line.split(": ");
    let mut wins_nums = card_nums.nth(1).unwrap().split(" | ");
    let wins: Vec<u32> = wins_nums
      .next()
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();
    let held: Vec<u32> = wins_nums
      .next()
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();

    Self { wins, held }
  }

  fn get_score(&self) -> u32 {
    let mut score: u32 = 0;

    for n in self.held.iter() {
      if self.wins.contains(n) {
        score = if score == 0 { 1 } else { score * 2 };
      }
    }

    score
  }

  fn get_wins_count(&self) -> usize {
    self.held.iter().filter(|n| self.wins.contains(n)).count()
  }
}

// This could be optimized, but there are only 214 cards so O(n^2) is fine.
fn solve_pt_2(cards: &Vec<Card>) -> u32 {
  let mut copies_buf: Vec<u32> = (0..cards.len()).map(|_| 1).collect();

  for i in 0..cards.len() {
    let wins = cards[i].get_wins_count();
    if wins > 0 {
      for j in i + 1..i + wins + 1 {
        copies_buf[j] += copies_buf[i];
      }
    }
  }
  copies_buf.iter().sum()
}

pub fn solve(input: String) -> (u32, u32) {
  let cards: Vec<Card> = input.lines().map(|line| Card::parse(line)).collect();

  let ans_1 = cards.iter().map(|card| card.get_score()).sum();

  let ans_2 = solve_pt_2(&cards);

  (ans_1, ans_2)
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::testing::test_with_input;

  #[test]
  fn test_scratchcards_sample() {
    test_with_input("scratchcards", solve, 13, 30);
  }
}
