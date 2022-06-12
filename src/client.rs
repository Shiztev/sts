/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::net::TcpStream;
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
    let _p: Vec<&str> = args.split("@").collect();
    let ssftp: Ssftp = Ssftp {
      username: "".to_string(),  // self.username = p[0].to_string();
      addr: "127.0.0.1:22".to_string(),  // self.addr = p[1].to_string();
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
    let tcp:TcpStream = TcpStream::connect(self.addr.as_str()).unwrap();
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
    println!();
    println!("{}", self.path);
    println!("{}", self.token);
  }

  /// Runs the provided command.
  pub fn run_cmd(cmd: String) {

  }

  /// Run an ssh command.
  fn ssh_cmd() {

  }

  /// Run an sftp command.
  fn sftp_cmd() {
    
  }
}
