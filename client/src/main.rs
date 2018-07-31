extern crate client;

use client::connection::Connection;
use client::logger::ConsoleLogger;
use client::menu::{Menu, MenuItem};
use client::{local, remote};

fn main() {
    let connection = Connection::to_container();
    connection.read_log();

    let mut menu = Menu::<ConsoleLogger, Connection>::new("SFTP Client", connection);

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
        "Put file onto remote server",
        remote::put_file,
    ));

    menu.insert(MenuItem::new(
        "Put multiple files onto remote server",
        remote::put_file_multi,
    ));

    menu.insert(MenuItem::new(
        "List local directories",
        local::list_directories,
    ));

    menu.insert(MenuItem::new(
        "Rename file on local machine",
        local::rename_file,
    ));

    menu.insert(MenuItem::new(
        "Change permissions on local directory",
        local::change_permission,
    ));

    menu.insert(MenuItem::new("Download remote file", remote::download_file));

    menu.insert(MenuItem::new(
        "Download multiple remote file",
        remote::download_file_multi,
    ));

    menu.insert(MenuItem::new("Delete remote file", remote::delete_file));

    menu.insert(MenuItem::new("Create remote file", remote::create_file));

    menu.insert(MenuItem::new("Delete remote file", remote::delete_file));

    menu.insert(MenuItem::new("Log off", |_| std::process::exit(0)));

    loop {
        menu();
        println!("");
    }
}
