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
    // Distingush and setup address and username
    let p: Vec<&str> = args.split("@").collect();
    let mut address: String = p[1].to_string();
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
    // Establish tcp connection
    println!("establishing {}'s connection at {}...", self.username, self.addr);
    let tcp: TcpStream = match TcpStream::connect(self.addr.as_str()) {
      Ok(t) => t,
      Err(e) => panic!("Problem establishing connection: {}", e),
    };

    // Set up session and authenticate
    self.sess.set_tcp_stream(tcp);
    self.sess.handshake().unwrap();
    // TODO: determine if a password is needed(?), if no password needed: s.userauth_agent(username).unwrap();
    let password: String = rpassword::prompt_password("password: ").unwrap();
    self.sess.userauth_password(self.username.as_str(), password.as_str()).unwrap();
    println!("connection successful");
  }

  /// Prompts user for input and prints server response.
  pub fn run(self) {
    let mut cmd:String = String::new();
    let mut exit_code: i32;

    while cmd != "exit" {
      // Read input
      cmd.clear();
      stdin().read_line(&mut cmd).expect("Problem reading user input");

      // Execute command and print output
      exit_code = self.run_cmd(&cmd);
      if exit_code != 0 {
        println!("Program ended with exit code {}", exit_code);
      }
      println!();
      println!("{}", self.path);
      println!("{}", self.token);
    }
  }

  /// Runs the provided command.
  fn run_cmd(&self, cmd: &String) -> i32 {
    let mut channel: ssh2::Channel;
    let mut output: String = String::new();
    let exit_code: i32;

    // Create channel
    match self.sess.channel_session() {
      Ok(c) => channel = c,
      Err(e) => panic!("Probelm creating channel: {}", e),
    }

    // Execute command
    channel.exec(&cmd).expect("Problem executing command");
    channel.read_to_string(&mut output).expect("Problem reading server response");
    println!("{}", output);

    // Cleanup and prep for next command
    channel.wait_close().expect("Problem waiting on server result");
    match channel.exit_status() {
      Ok(n) => exit_code = n,
      Err(e) => panic!("Problem getting exit status: {}", e)
    }

    // TODO: update self.path
    exit_code
  }
}
