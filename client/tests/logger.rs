extern crate client;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use client::connection::Connection;
use client::input::MockInput;
use client::local;
use client::menu::{Menu, MenuItem};

fn create_menu(input: MockInput) -> Menu {
    let mut connection = Connection::to_container(input);

    let mut menu = Menu::new("SFTP Client", connection);

    menu.insert(MenuItem::new(
        "List local directories",
        local::list_directories,
    ));

    menu.show();

    menu
}

#[test]
fn creates_file() {
    let mut connection = Connection::to_container(MockInput::new());
    let menu = Menu::new("SFTP Client", connection);

    let path = Path::new("logger.txt");
    assert!(local::file_exists(path));
}

#[test]
fn logs_history() {
    let mut input = MockInput::new();
    input.set_path(Path::new("/"));

    create_menu(input);

    let mut log_file = File::open(Path::new("logger.txt")).unwrap();
    let mut contents = String::new();
    log_file.read_to_string(&mut contents).unwrap();
    let split = contents.split(" ").collect::<Vec<_>>();

    let mut log_entry = String::new();
    for part in split[1..].iter() {
        log_entry.push_str(&format!("{} ", part));
    }

    log_entry = log_entry.trim().to_string();
    assert!(log_entry == "User listed local directories at \"/\"");
}
