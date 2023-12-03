mod cubes;
mod engine;
mod trebuchet;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    panic!(
      "ERROR: Call with the name of the challenge, e.g. `cargo run trebuchet` for trebuchet.rs"
    );
  }

  if let Some(challenge_name) = args.get(1) {
    println!("Showing solutions for {}", challenge_name);
    match challenge_name.as_str() {
      "trebuchet" => display_ans(trebuchet::solve(get_input(challenge_name))),
      "cubes" => display_ans(cubes::solve(get_input(challenge_name))),
      "engine" => display_ans(engine::solve(get_input(challenge_name))),
      _ => panic!("ERROR: Unrecognized challenge name: `{}`", challenge_name),
    }
  }
}

fn get_input(challenge_name: &str) -> String {
  std::fs::read_to_string(format!("inputs/{}.txt", challenge_name))
    .expect("Could not find input file.")
}

fn display_ans<U, V>(ans: (U, V))
where
  U: std::fmt::Debug,
  V: std::fmt::Debug,
{
  let (a, b) = ans;
  println!("With solutions:\nans_1 = {:?}\nans_2 = {:?}", a, b);
}
