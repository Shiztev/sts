use ssh2::{Session, Channel, ScpFileStat};
use std::{net::TcpStream, io::{stdin, Read, Write, BufReader}, path::Path, fs::{File, self}};
use rpassword;

use crate::client::Ssftp;

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
  
    // Create channel
    let mut channel: ssh2::Channel;
    match self.sess.channel_session() {
      Ok(c) => channel = c,
      Err(e) => panic!("Probelm creating channel: {}", e),
    }
    channel.shell().expect("Should be able to create shell");
    match channel.shell() {
      Ok(_) => (),
      Err(e) => panic!("Problem creating shell: {}", e),
    }
    println!("connection successful");
  }


  /// Prompts user for input and prints server response.
  pub fn run(mut self) {
    let mut cmd:String = String::new();
    let mut exit_code: i32;
    let mut channel: ssh2::Channel;

    // Create channel
    match self.sess.channel_session() {
      Ok(c) => channel = c,
      Err(e) => panic!("Probelm creating channel: {}", e),
    }
    channel.shell();

    loop {
      //self.update_prompt();
      println!("{}\n{}", self.path, self.token);

      // Read input and trim
      cmd.clear();
      stdin().read_line(&mut cmd).expect("Problem reading user input");
      cmd = cmd.trim_end().to_owned();

      if cmd == "exit" {
        return;
      }

      // Execute command and print output
      /*
      exit_code = self.run_cmd(&cmd, &mut channel);
      if exit_code != 0 {
        println!("Program ended with exit code {}", exit_code);
      }
      */
    }
  }


  /// Runs the provided command.
  fn run_cmd(&mut self, cmd: &String, &mut channel: ssh2::Channel) -> i32 {
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

    l = self.get_path(parts);

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
}