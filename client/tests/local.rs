extern crate client;

use client::connection::Connection;
use client::input::MockInput;
use client::local;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[test]
fn change_permissions() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file20.txt");
    input.set_path(&path);
    input.set_string("444");

    let mut connection = Connection::to_container(input);

    let f = File::create("/demo_file20.txt").unwrap();
    let metadata = f.metadata().unwrap();
    assert_eq!(false, metadata.permissions().readonly());
    local::change_permission(&mut connection);
    assert_eq!(false, metadata.permissions().readonly());
    local::delete_file(&mut connection);
}

#[test]
fn rename_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file21.txt");
    let new_path = PathBuf::from("/demo_file22.txt");
    input.set_path(&path);
    input.set_path(&new_path);

    let mut connection = Connection::to_container(input);

    File::create("/demo_file21.txt").unwrap();
    local::rename_file(&mut connection);
    assert_eq!(Path::new("/demo_file22.txt").exists(), false);
    local::delete_file(&mut connection);
}
