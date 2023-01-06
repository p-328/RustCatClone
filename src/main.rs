use std::fs::read_to_string;

fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();
  if let None = args.get(1) {
    return Err("Usage: ./cat_clone <filename>".to_string())
  }
  let result = match read_to_string(args.get(1).unwrap()) {
    Ok(text) => text,
    Err(e) => format!("{:?}", e)
  };
  println!("{:?}", result);
  Ok(())
}