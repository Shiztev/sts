mod client;
use client::Ssftp;
use std::env;

fn main() {
  let args:Vec<String> = env::args().collect();
  
  if args.len() != 2 {
    panic!("usage: ssftp <username>@<ip/domain name/...>");

  } else {
    let mut ssftp: Ssftp = Ssftp::new(&args[1]);
    ssftp.ssh_init();
    ssftp.run();
  }
}
