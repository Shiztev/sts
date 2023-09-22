use std::env;
use sts::Sts;
mod sts;

fn main() {
	let args:Vec<String> = env::args().collect();
	let mut sts: Sts;
  
	if args.len() != 2 {
		println!("usage: sts <username>@<ip/domain name/...>");

	} else {
		//sts = Sts::new(&args[1]);
		sts::threaded_full_test(&args[1]);
		//sts.run();
	}
}
