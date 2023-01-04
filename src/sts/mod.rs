/// Starts and manages both an SSH and a SFTP connection, running user commands.


pub struct Sts {
  username: String,
  addr: String,
  sess: Session,
  path: String,
  home: String,
  token: String
}


mod Estab {

}


impl Sts {
  /// Initialize an Sts instance from the given username and address.
  pub fn new(args: &String) -> Sts {
    // Distingush and setup address and username
    let p: Vec<&str> = args.split("@").collect();
    let mut address: String = p[1].to_string();
    let u_name:String = p[0].to_string();
    let loc = format!("/home/{}", u_name);
    address.push_str(":22");

    let sts: Sts = Sts {
      username: u_name.clone(),
      addr: address,
      sess: Session::new().unwrap(),
      path: String::from(&loc),
      home: String::from(&loc),
      token: String::from("$")
    };

    println!("initializing {}'s connection to {}...", ssftp.username, ssftp.addr);
    sts
  }
}