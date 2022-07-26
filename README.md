# ssftp
A CLI application that combines SSH and SFTP functionality to simplify and unify the two services.


## Requirements
Rust >= 1.60.0



# Commands
First, install the dependancies: `cargo install --path .`

To run: `cargo run -- <username>@<ip/hostname/...>`

To build then run: `cargo build`

`ssftp <username>@<ip/hostname/...>`


# Dependancies and Credits
<a target="_blank" href="https://github.com/sfackler/rust-openssl">openssl library</a>

<a target="_blank" href="https://github.com/alexcrichton/ssh2-rs">ssh2-rs library</a>

<a target="_blank" href="https://github.com/conradkleinespel/rpassword">rpassword library</a>
