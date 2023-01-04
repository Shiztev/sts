use std::env;
use sts::Sts;
//mod ssftp;
mod sts;


fn main() {
  let args:Vec<String> = env::args().collect();
  
  if args.len() != 2 {
    panic!("usage: ssftp <username>@<ip/domain name/...>");

  } else {
    let mut sts: Sts = Sts::new(&args[1]);
    sts.run();
    //let mut ssftp: Ssftp = Ssftp::new(&args[1]);
    //ssftp.ssh_init();
    //ssftp.run();
  }
}
