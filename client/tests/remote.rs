extern crate client;

use std::path::PathBuf;

use client::connection::Connection;
use client::input::MockInput;
use client::remote;

#[test]
fn create_and_delete_directory() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_directory_1");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_directory(&mut connection);
    assert!(remote::directory_exists(&mut connection, &path));

    remote::delete_directory(&mut connection);
    assert!(!remote::directory_exists(&mut connection, &path));
}

#[test]
fn create_and_delete_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file1.txt");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_file(&mut connection);
    assert!(remote::file_exists(&mut connection, &path));

    remote::delete_file(&mut connection);
    assert!(!remote::file_exists(&mut connection, &path));
}

#[test]
fn change_permissions() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file2.txt");
    input.set_path(&path);
    input.set_string("777");

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_file(&mut connection);
    assert!(remote::file_permissions(&mut connection, &path) == 33188);

    remote::change_permission(&mut connection);
    assert!(remote::file_permissions(&mut connection, &path) == 33279);

    remote::delete_file(&mut connection);
}

#[test]
fn rename_file() {
    let mut input = MockInput::new();
    let source_path = PathBuf::from("/demo_file3.txt");
    let destination_path = PathBuf::from("/demo_file3_renamed.txt");

    input.set_paths(&[&source_path, &destination_path]);
    input.set_prompt_paths(&[&source_path, &destination_path]);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_file(&mut connection);
    assert!(remote::file_exists(&mut connection, &source_path));

    remote::rename_file(&mut connection);
    assert!(!remote::file_exists(&mut connection, &source_path));
    assert!(remote::file_exists(&mut connection, &destination_path));

    remote::delete_file(&mut connection);
    assert!(!remote::file_exists(&mut connection, &destination_path));
}
