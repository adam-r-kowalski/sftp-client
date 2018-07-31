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

pub fn rename_file(_: &Connection) -> String {
    let old_path = input::prompt_path("Enter old path: ");
    let new_path = input::prompt_path("Enter new path: ");
    match fs::rename(&old_path, &new_path) {
        Ok(_)   => format!("Renamed {:?} to {:?}.", old_path, new_path),
        Err(e)  => e.to_string(),
    }
}

