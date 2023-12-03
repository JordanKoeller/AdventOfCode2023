mod cubes;
mod engine;
mod trebuchet;

fn main() {
  println!("Hello, world!");
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    panic!("Call with the name of the challenge, e.g. `trebuchet` for trebuchet.rs");
  }

  if let Some(challenge_name) = args.get(1) {
    let input: String = std::fs::read_to_string(format!("inputs/{}.txt", challenge_name))
      .expect("Could not find input file.");
    match challenge_name.as_str() {
      "trebuchet" => trebuchet::solve(input),
      "cubes" => cubes::solve(input),
      "engine" => engine::solve(input),
      _ => panic!("Unrecognized challenge name: `{}`", challenge_name),
    }
  }
}
