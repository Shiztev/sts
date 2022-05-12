/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::net::TcpStream;

/// Start an SSH and SFTP connection, and loop while executing user commands.
pub fn run() {
  println!("Run is running!");

  let addr: &str = "127.0.0.1:22";

  // TODO: read ip from CLI

  // Connect to SSH dest
  let tcp: TcpStream = TcpStream::connect(addr).unwrap();

  // Start session
  let mut s: Session = Session::new().unwrap();
  s.set_tcp_stream(tcp);  // provide a tcp stream to route communication through
  s.handshake().unwrap();  // confirm conneciton
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