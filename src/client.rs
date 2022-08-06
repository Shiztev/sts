/// Starts and manages both an SSH and a SFTP connection, running user commands.

use ssh2::Session;
use std::{net::TcpStream, io::{stdin, Read, Write}, path::Path};
use rpassword;

pub struct Ssftp {
  username: String,
  addr: String,
  sess: Session,
  path: String,
  home: String,
  token: String
}

impl Ssftp {
  /// Initialize an Ssftp instance from the given username and address.
  pub fn new(args: &String) -> Ssftp {
    // Distingush and setup address and username
    let p: Vec<&str> = args.split("@").collect();
    let mut address: String = p[1].to_string();
    let u_name:String = p[0].to_string();
    let loc = format!("/home/{}", u_name);
    address.push_str(":22");

    let ssftp: Ssftp = Ssftp {
      username: u_name.clone(),
      addr: address,
      sess: Session::new().unwrap(),
      path: String::from(&loc),
      home: String::from(&loc),
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
  pub fn run(mut self) {
    let mut cmd:String = String::new();
    let mut exit_code: i32;

    while cmd.trim_end() != "exit" {
      //self.update_prompt();
      println!("{}\n{}", self.path, self.token);

      // Read input
      cmd.clear();
      stdin().read_line(&mut cmd).expect("Problem reading user input");

      // Execute command and print output
      cmd = format!("cd {} && {} && pwd", self.path, cmd.trim_end());
      exit_code = self.run_cmd(&cmd);
      if exit_code != 0 {
        println!("Program ended with exit code {}", exit_code);
      }
    }
  }

  /// Runs the provided command.
  fn run_cmd(&mut self, cmd: &String) -> i32 {
    let mut channel: ssh2::Channel;
    let mut output: String = String::new();
    let exit_code: i32;
    let parts: Vec<&str>;

    // Create channel
    match self.sess.channel_session() {
      Ok(c) => channel = c,
      Err(e) => panic!("Probelm creating channel: {}", e),
    }

    // Check command against internal functionality
    parts = cmd.split(" ").collect();
    match parts[0] {
      "put" => return self.upload(parts),
      "get" => return self.download(parts),
      _ => ()
    }

    // Execute command and parse out path
    channel.exec(&cmd).expect("Problem executing command");
    channel.read_to_string(&mut output).expect("Problem reading server response");
    let v3: Vec<&str> = output.split("\n").collect();

    self.path = v3[v3.len() - 2].to_string();
    let l = self.path.len();
    if self.path == self.home {
      self.path = String::from("~");
    }

    println!("{}", &output[..output.len() - l - 1]);

    // Cleanup and prep for next command
    channel.wait_close().expect("Problem waiting on server result");
    match channel.exit_status() {
      Ok(n) => exit_code = n,
      Err(e) => panic!("Problem getting exit status: {}", e)
    }

    exit_code
  }

  // Upload a file.
  fn upload(&self, parts: Vec<&str>) -> i32 {
    let path: &Path;
    let mode: i32;
    let size: u64;
    let len = parts.len();

    if len < 2 {
      println!("Usage: put <flags> <local file> <OPTIONAL: remote write path>");
      return 1;
    } else {
      return 1;
    }

    // TODO: get stuff from parts var
    path = Path::new("");
    mode = 0o644;
    size = 10;

    let mut remote_file = self.sess.scp_send(path, mode, size, None).unwrap();
    remote_file.write(b"1234567890").unwrap();
    // Close the channel and wait for the whole content to be tranferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
    0
  }

  // Download a file.
  fn download(&self, parts: Vec<&str>) -> i32 {
    println!("Get command is not yet implemented");
    1
  }
}
