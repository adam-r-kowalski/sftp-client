extern crate ssh2;
extern crate client;

use client::menu::{Menu, MenuItem};
use client::remote;

use std::net::TcpStream;
use ssh2::Session;

fn main() {
    let tcp = TcpStream::connect("server:22").unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_password("root", "root").unwrap();

    let sftp = session.sftp().unwrap();

    let mut menu = Menu::new("SFTP Client", sftp);

    menu.insert(MenuItem::new(
        "Create directory on remote server",
        remote::create_directory));

    menu.insert(MenuItem::new(
        "Delete directory on remote server",
        remote::delete_directory));

    menu.insert(MenuItem::new(
        "List directories on remote server",
        remote::list_directories));

    menu();
}
