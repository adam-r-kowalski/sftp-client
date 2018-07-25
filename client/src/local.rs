use std::fs;

use connection::Connection;
use input;

pub fn list_directories(_: &Connection) -> String {
    let path = input::path();
    fs::read_dir(&path)
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.unwrap().path()));
    format!("User listed local directories at {:?}", path)
}
