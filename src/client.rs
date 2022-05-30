/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::net::TcpStream;
use rpassword;

// TODO: INSTALL libssl.so.1.1, if need be see: https://github.com/openssl/openssl/issues/1740

/// Start an SSH and SFTP connection, and loop while executing user commands.
pub fn run(args: &str) {
  // reference: https://docs.rs/ssh2/latest/ssh2/index.html
  println!("Run is running!");

  // TODO: read ip and username from CLI
  let _p: Vec<&str> = args.split("@").collect();

  // let username: &str = p[0];
  let username: &str = "";
  // let addr: &str = p[1];
  let addr: &str = "127.0.0.1:22";


  let p_needed: bool = true;

  // Connect to SSH dest
  let tcp: TcpStream = TcpStream::connect(addr).unwrap();

  // Start session
  let mut s: Session = Session::new().unwrap();
  s.set_tcp_stream(tcp);  // provide a tcp stream to route communication through
  s.handshake().unwrap();  // confirm conneciton

  // determine if password is needed

  if p_needed {
    // rpassword docs: https://docs.rs/rpassword/6.0.1/rpassword/
    let password: String = rpassword::prompt_password("password: ").unwrap();  // read password
    s.userauth_password(username, password.as_str()).unwrap();  // auth

  } else {
    s.userauth_agent(username).unwrap();  // auth
  }


  // test code
  let mut a = s.agent().unwrap();

  // Check agent
  a.connect().unwrap();
  a.list_identities().unwrap();
  
  for i in a.identities().unwrap() {
    println!("{}", i.comment());
    let _pk = i.blob();
  }
}

/// SSH commands
mod ssh {

}

/// SFTP commands
mod sftp {

}
