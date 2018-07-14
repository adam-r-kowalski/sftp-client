use ssh2::Session;
use std::path::PathBuf;
use std::io;
use std::io::Write;

use connection::Connection;

fn get_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    String::from(input_text.trim())
}

fn get_path_buf() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(get_string("\nEnter path:"));
    path
}

pub fn list_directories(connection: &Connection) {
    connection.sftp().readdir(get_path_buf().as_path())
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.0));
}

pub fn create_directory(connection: &Connection) {
    connection.sftp().mkdir(get_path_buf().as_path(), 0).unwrap()
}

pub fn delete_directory(connection: &Connection) {
    connection.sftp().rmdir(get_path_buf().as_path()).unwrap()
}

pub fn rename_file(connection: &Connection) {
    let source = get_path_buf();
    let destination = get_path_buf();
    connection.sftp().rename(&source, &destination, None).unwrap();
}
