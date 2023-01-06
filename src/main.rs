use std::fs::read_to_string;


fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();
  if let None = args.get(1) {
    return Err("Usage: cargo run <filename>".to_string())
  }
  if let Err(e) = read_to_string(args.get(1).unwrap()) {
    let msg = format!("{:?}", e);
    return Err(msg)
  }
  println!("{}", read_to_string(args.get(1).unwrap()).unwrap());
  Ok(())
}