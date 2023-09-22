/// Starts and manages both an SSH and a SFTP connection, running user commands.

use std::io::{stdin, Read, Write};

use ssh2::{Session, Channel};

pub struct Sts {
	username: String,
	addr: String,
	sess: Session,
	channel: Channel,
}


// NOTE: SEE https://github.com/alexcrichton/ssh2-rs/issues/186
// and https://github.com/alexcrichton/ssh2-rs/issues/128
// YOU NEED TO REWORK THE PROGRAM


mod estab {
	use std::net::TcpStream;
	use ssh2::{Session, Channel};

	/// Create an ssh session.
	pub fn estab_connection(username: &String, addr: &String) -> (Session, Channel) {
		println!("initializing {}'s connection to {}...", username, addr);
		let sess: Session = estab_tcp(&username, &addr);

		println!("establishing {}'s connection at {}...", username, addr);
		let channel: Channel = estab_shell(&sess);

		println!("connection successful");
		(sess, channel)
	}

	/// Establish an ssh session via TCP.
	fn estab_tcp(username: &String, addr: &String) -> Session {
		// Establish tcp connection
		let mut sess: Session = Session::new().unwrap();
		let tcp: TcpStream = match TcpStream::connect(addr.as_str()) {
			Ok(t) => t,
			Err(e) => panic!("Problem establishing connection: {}", e),
		};

		// Set up session and authenticate
		sess.set_tcp_stream(tcp);
		sess.handshake().unwrap();

		// TODO: determine if a password is needed(?), if no password needed: s.userauth_agent(username).unwrap();
		let password: String = rpassword::prompt_password("password: ").unwrap();
		sess.userauth_password(username.as_str(), password.as_str()).unwrap();

		sess
	}

	/// Establish an ssh Shell channel.
	fn estab_shell(sess: &Session) -> Channel {
		let mut channel: Channel;
		match sess.channel_session() {
			Ok(c) => channel = c,
			Err(e) => panic!("Probelm creating channel: {}", e),
		}

		channel.shell().expect("Should be able to create shell");

		//match channel.shell() {
		//  Ok(_) => (),
		//  Err(e) => panic!("Problem creating shell: {}", e),
		//}

		channel
	}
}


use std::net::TcpStream;
extern crate crossbeam;
use crossbeam::channel::{unbounded, TryRecvError};
use std::thread;
use std::io::prelude::*;
pub fn threaded_full_test(args: &String) {

	/***** get username and addr ******/
	let p: Vec<&str> = args.split("@").collect();
	let mut addr: String = p[1].to_string();
	let username: String = p[0].to_string();
	addr.push_str(":22");


	/***** set up tcp and session *****/
	let mut sess: Session = Session::new().unwrap();
	let tcp: TcpStream = TcpStream::connect(addr.as_str()).unwrap();

	// Set up session and authenticate
	sess.set_tcp_stream(tcp);
	sess.handshake().unwrap();

	/***** password *****/
	// TODO: determine if a password is needed(?), if no password needed: s.userauth_agent(username).unwrap();
	let password: String = rpassword::prompt_password("password: ").unwrap();
	sess.userauth_password(username.as_str(), password.as_str()).unwrap();

	/***** get channel and set to shell *****/
	let mut channel: Channel = sess.channel_session().unwrap();
	//channel.request_pty("xterm", None, Some((80, 24, 0, 0))).unwrap();
	channel.shell().unwrap();

	sess.set_blocking(false);
	
	/***** exec commands *****/
	let (trx, rev) = unbounded();
	thread::spawn(move || loop {
		let stdin = std::io::stdin();
		let mut line = String::new();
		stdin.read_line(&mut line).unwrap();
		trx.send(line).unwrap();
	});

	loop {
		let mut buf = vec![0; 4096];
		match channel.read(&mut buf) {
			Ok(_) => {
				let s = String::from_utf8(buf).unwrap();
				println!("{}", s);
			}
			Err(e) => {
				if e.kind() != std::io::ErrorKind::WouldBlock {
					println!("{}", e);
				}
			}
		}

		if !rev.is_empty() {
			match rev.try_recv() {
				Ok(line) => {
					let cmd_string = line + "\n";
					channel.write(cmd_string.as_bytes()).unwrap();
					channel.flush().unwrap();
				}
				Err(TryRecvError::Empty) => {
					println!("{}", "empty");
				}
				Err(TryRecvError::Disconnected) => {
					println!("{}", "disconnected");
				}
			}
		}
	}
}


impl Sts {
	/// Initialize an Sts instance from the given username and address.
	pub fn new(args: &String) -> Sts {
		// Distingush and setup address and username
		let p: Vec<&str> = args.split("@").collect();
		let mut address: String = p[1].to_string();
		let u_name: String = p[0].to_string();
		address.push_str(":22");

		let conn: (Session, Channel) = estab::estab_connection(&u_name, &address);
		let sts: Sts = Sts {
			username: u_name.clone(),
			addr: address,
			sess: conn.0,
			channel: conn.1,
		};

		sts
	}

	pub fn test(&mut self) {




		let mut channel: Channel = self.sess.channel_session().expect("error creating test channel");
		channel.request_pty("xterm", None, Some((80, 24, 0, 0))).unwrap();
		channel.shell().expect("error making channel a shell");

		channel.exec("ls").unwrap();

		let mut s: String = String::new();
		channel.read_to_string(&mut s).unwrap();
		println!("{}", s);

		channel.exec("diff ~/.bashrc ~/.vimrc").unwrap();
		channel.exec("pwd").unwrap();
		channel.read_to_string(&mut s).unwrap();
		println!("{}", s);

		channel.wait_close().unwrap();
	}

	pub fn run(&mut self) {
		let mut cmd: String = String::new();
		let mut exit_code: i32;

		loop {
			cmd.clear();
			stdin().read_line(&mut cmd).expect("Could not read user input from stdin");
			cmd = cmd.trim_end().to_owned();

			if cmd == "exit" {
				self.close();
				return;
			}

			exit_code = self.run_cmd(&cmd);

			match self.channel.exit_status() {
				Ok(n) => exit_code = n,
				Err(e) => panic!("{}", e),
			}
		}
	}

	fn run_cmd(&mut self, cmd: &String) -> i32 {
		let mut buf: String = String::new();
		self.channel.exec(&cmd).expect("Problem executing command");
		self.channel.read_to_string(&mut buf).expect("Problem reading server response to string");
		println!("{}", buf);
		0
	}

	fn close(&mut self) {
		self.channel.close().expect("Problem closing channel");
		return;
	}
}
