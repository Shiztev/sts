mod client;
use std::env;

fn main() {
  let args:Vec<String> = env::args().collect();
  
  if args.len() != 1 {
    panic!("usage: ssftp <username>@<ip/domain name/...>");
  }

  client::run(&args[0].as_str());
}
