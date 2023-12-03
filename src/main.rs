use std::fmt::Debug;
mod cubes;
mod engine;
mod trebuchet;

fn solve<U, V, F>(challenge_name: &str, func: F)
where
  U: Debug,
  V: Debug,
  F: Fn(String) -> (U, V),
{
  // Grab file input
  let input = std::fs::read_to_string(format!("inputs/{}.txt", challenge_name))
    .expect("Could not find input file.");

  // Print prelude, run code, print results
  println!("Showing solutions for {}", challenge_name);
  let (a, b) = func(input);
  println!("With solutions:\nans_1 = {:?}\nans_2 = {:?}", a, b);
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    panic!(
      "ERROR: Call with the name of the challenge, e.g. `cargo run trebuchet` for trebuchet.rs"
    );
  }

  if let Some(challenge_name) = args.get(1) {
    match challenge_name.as_str() {
      "trebuchet" => solve(&challenge_name, trebuchet::solve),
      "cubes" => solve(&challenge_name, cubes::solve),
      "engine" => solve(&challenge_name, engine::solve),
      _ => panic!("ERROR: Unrecognized challenge name: `{}`", challenge_name),
    }
  }
}
