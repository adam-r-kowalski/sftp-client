extern crate ssh2;
extern crate client;

use client::menu::{Menu, MenuItem};

use std::net::TcpStream;
use ssh2::Session;

fn main() {
    let tcp = TcpStream::connect("server:22").unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_password("root", "root").unwrap();

    let sftp = session.sftp().unwrap();

    let mut menu = Menu::new("SFTP Client", sftp);

    menu.push(MenuItem::new(
        "Create Directory On Remote Server",
        Box::new(|_| println!("hello"))));

    menu.push(MenuItem::new(
        "Delete Directory on Remote Server",
        Box::new(|_| println!("hello"))));

    print!("{}", menu);
    menu.get_input();
}
