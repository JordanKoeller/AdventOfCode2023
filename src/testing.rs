// This file has some helpers for testing. Does not contain source code for ana actual challenge.

use std::fmt::Debug;
use std::fs;

pub fn test_with_input<U, V, F>(challenge_name: &str, func: F, expected_1: U, expected_2: V)
where
  U: Debug + Eq,
  V: Debug + Eq,
  F: Fn(String) -> (U, V),
{
  let file_name = format!("inputs/{}_sample.txt", challenge_name);
  let input =
    fs::read_to_string(&file_name).expect(&format!("Could not find file with name {}", file_name));

  let (ans_1, ans_2) = func(input);

  assert_eq!(ans_1, expected_1, "Failed on Part 1");
  assert_eq!(ans_2, expected_2, "Failed on Part 2");
}
