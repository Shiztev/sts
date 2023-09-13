use std::env;
use sts::Sts;
mod sts;
use crate::sts::full_test;

fn main() {
	let args:Vec<String> = env::args().collect();
	let mut sts: Sts;
  
	if args.len() != 2 {
		println!("usage: sts <username>@<ip/domain name/...>");

	} else {
		//sts = Sts::new(&args[1]);
		full_test(&args[1]);
		//sts.run();
	}
}
