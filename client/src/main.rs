extern crate client;
extern crate ssh2;

use client::menu::{Menu, MenuItem};
use client::remote;

use ssh2::Session;
use std::net::TcpStream;

fn main() {
    let tcp = TcpStream::connect("server:22").unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_password("root", "root").unwrap();

    let sftp = session.sftp().unwrap();

    let mut menu = Menu::new("SFTP Client", sftp);

    menu.insert(MenuItem::new(
        "Create directory on remote server",
        remote::create_directory,
    ));

    menu.insert(MenuItem::new(
        "Delete directory on remote server",
        remote::delete_directory,
    ));

    menu.insert(MenuItem::new(
        "List directories on remote server",
        remote::list_directories,
    ));

    menu.insert(MenuItem::new(
        "Rename file or direcory on remote server",
        remote::rename_file,
    ));

    menu.insert(MenuItem::new(
        "Log off remote server",
        |_| std::process::exit(0)
    ));

    loop {
        menu();
    }
}
