mod client;
use std::env;

fn main() {
  let args:Vec<String> = env::args().collect();
  client::run(args);
}
