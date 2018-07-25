extern crate client;
extern crate ssh2;

use client::connection::Connection;
use client::menu::{Menu, MenuItem};
use client::{local, remote};

fn main() {
    let connection = Connection::to_container();

    let mut menu = Menu::new("SFTP Client", connection);

    menu.insert(MenuItem::new(
        "List remote directories",
        remote::list_directories,
    ));

    menu.insert(MenuItem::new(
        "Create remote directory",
        remote::create_directory,
    ));

    menu.insert(MenuItem::new(
        "Delete remote directory",
        remote::delete_directory,
    ));

    menu.insert(MenuItem::new(
        "Rename remote file or directory",
        remote::rename_file,
    ));

    menu.insert(MenuItem::new(
        "List local directories",
        local::list_directories,
    ));

    menu.insert(MenuItem::new("Delete remote file", remote::delete_file));

    menu.insert(MenuItem::new("Create remote file", remote::create_file));

    menu.insert(MenuItem::new("Log off", |_| std::process::exit(0)));
    loop {
        menu();
        println!("");
    }
}
