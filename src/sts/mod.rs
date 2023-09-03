/// Starts and manages both an SSH and a SFTP connection, running user commands.

use std::io::{stdin, Read};

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
	use std::io::Read;

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
