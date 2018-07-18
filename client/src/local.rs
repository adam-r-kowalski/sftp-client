use std::fs;

use connection::Connection;
use input;


pub fn list_directories(_: &Connection) {
    fs::read_dir(&input::path().as_path()).unwrap()
	.into_iter()
        .for_each(|d| println!("{:?}" , d.unwrap().path()));
}

