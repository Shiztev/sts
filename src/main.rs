use std::{env, net::TcpStream};
use ssftp::ssftp::Ssftp;
use ssh2::{Session, Channel, ScpFileStat};
mod ssftp;
mod sts;


/// Runs the provided command.
fn run_cmd(&mut sftp: Ssftp, cmd: &String, &mut channel: ssh2::Channel) -> i32 {
  let mut output: String = String::new();
  let command: String;
  let exit_code: i32;
  let mut parts: Vec<&str>;
  let l: usize;

  // Check command against internal functionality
  parts = cmd.split(" ").collect();
  /*
  match parts[0] {
    "put" => return self.upload(parts),
    "get" => return self.download(parts),
    _ => ()
  }
  */

  // Format and execute command
  //command = format!("cd {} && {} && pwd", self.path, cmd);
  channel.exec(&command).expect("Problem executing command");
  channel.read_to_string(&mut output).expect("Problem reading server response");
  parts = output.split("\n").collect();

  l = sftp.get_path(parts);

  println!("{}", &output[..output.len() - l - 1]);

  // Cleanup and prep for next command
  /*
  channel.wait_close().expect("Problem waiting on server result");
  match channel.exit_status() {
    Ok(n) => exit_code = n,
    Err(e) => panic!("Problem getting exit status: {}", e)
  }
  */

  //exit_code
}

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
