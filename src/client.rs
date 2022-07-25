/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::{net::TcpStream, io::{stdin, Read}};
use rpassword;

pub struct Ssftp {
  username: String,
  addr: String,
  sess: Session,
  path: String,
  token: String
}

impl Ssftp {
  /// Initialize an Ssftp instance from the given username and address.
  pub fn new(args: &String) -> Ssftp {
    let p: Vec<&str> = args.split("@").collect();
    let mut address = p[1].to_string();
    address.push_str(":22");

    let ssftp: Ssftp = Ssftp {
      username: p[0].to_string(),
      addr: address,
      sess: Session::new().unwrap(),
      path: String::from(""),
      token: String::from("$")
    };

    println!("initializing {}'s connection to {}...", ssftp.username, ssftp.addr);
    ssftp
  }

  /// Start an ssh connection via tcp to the instances address. Prompts for a password.
  pub fn ssh_init(&mut self) {
    println!("establishing {}'s connection at {}...", self.username, self.addr);
    let tcp_result = TcpStream::connect(self.addr.as_str());

    let tcp = match tcp_result {
      Ok(t) => t,
      Err(e) => panic!("Problem establishing connection: {}", e),
    };

    self.sess.set_tcp_stream(tcp);
    self.sess.handshake().unwrap();
    // TODO: determine if a password is needed(?)
    // rpassword docs: https://docs.rs/rpassword/6.0.1/rpassword/
    let password: String = rpassword::prompt_password("password: ").unwrap();  // read password
    self.sess.userauth_password(self.username.as_str(), password.as_str()).unwrap();  // auth
    // if no password needed: s.userauth_agent(username).unwrap();
    println!("connection successful");
  }

  /// Prompts user for input and prints server response.
  pub fn run(self) {
    let mut buf:String = String::new();
    let mut channel: ssh2::Channel;
    let mut exit_code: i32;

    while buf != "exit" {
      // Read input
      buf.clear();
      stdin().read_line(&mut buf).expect("Problem reading user input");

      // Create channel
      match self.sess.channel_session() {
        Ok(c) => channel = c,
        Err(e) => panic!("Probelm creating channel: {}", e),
      }

      // Execute command
      channel.exec(&buf).expect("Problem executing command");
      channel.read_to_string(&mut buf).expect("Problem reading server response");
      println!("{}", buf);

      // Cleanup and prep for next command
      channel.wait_close().expect("Problem waiting on server result");
      match channel.exit_status() {
        Ok(n) => exit_code = n,
        Err(e) => panic!("Problem getting exit status: {}", e)
      }

      if exit_code != 0 {
        println!("Program ended with exit code {}", exit_code);
      }
      println!();
      println!("{}", self.path);
      println!("{}", self.token);
    }
  }

  ///// Runs the provided command.
  //fn run_cmd(cmd: String) {

  //}
}
