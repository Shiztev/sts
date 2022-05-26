/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::net::TcpStream;

/// Start an SSH and SFTP connection, and loop while executing user commands.
pub fn run() {
  // reference: https://docs.rs/ssh2/latest/ssh2/index.html
  println!("Run is running!");

  let addr: &str = "127.0.0.1:22";

  // TODO: read ip from CLI
  let username: &str = "";

  let p_needed: bool = true;

  // Connect to SSH dest
  let tcp: TcpStream = TcpStream::connect(addr).unwrap();

  // Start session
  let mut s: Session = Session::new().unwrap();
  s.set_tcp_stream(tcp);  // provide a tcp stream to route communication through
  s.handshake().unwrap();  // confirm conneciton

  // determine if password is needed
  if p_needed {
    s.userauth_password(username, password).unwrap();

  } else {
    s.userauth_agent(username).unwrap();
  }


  // test code
  let mut a = s.agent().unwrap();

  // Check agent
  a.connect().unwrap();
  a.list_identities().unwrap();
  
  for i in a.identities().unwrap() {
    println!("{}", i.comment());
    let pk = i.blob();
  }
}

/// SSH commands
mod ssh {

}

/// SFTP commands
mod sftp {

}