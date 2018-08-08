extern crate client;

use std::path::PathBuf;

use client::connection::Connection;
use client::input::MockInput;
use client::{local, remote};
use std::process::Command;

#[test]
fn create_directory() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_directory_1");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_directory(&mut connection);
    assert!(remote::directory_exists(&mut connection, &path));

    remote::delete_directory(&mut connection);
}

#[test]
fn delete_directory() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_directory_2");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));
    remote::create_directory(&mut connection);

    remote::delete_directory(&mut connection);
    assert!(!remote::directory_exists(&mut connection, &path));
}

#[test]
fn create_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file1.txt");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_file(&mut connection);
    assert!(remote::file_exists(&mut connection, &path));

    remote::delete_file(&mut connection);
}

#[test]
fn delete_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file2.txt");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));
    remote::create_file(&mut connection);

    remote::delete_file(&mut connection);
    assert!(!remote::file_exists(&mut connection, &path));
}

#[test]
fn change_permissions() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file3.txt");
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
    let source_path = PathBuf::from("/demo_file4.txt");
    let destination_path = PathBuf::from("/demo_file4_renamed.txt");

    input.set_paths(&[&source_path, &destination_path]);
    input.set_prompt_paths(&[&source_path, &destination_path]);

    let mut connection = Connection::to_container(Box::new(input));
    remote::create_file(&mut connection);

    remote::rename_file(&mut connection);
    assert!(!remote::file_exists(&mut connection, &source_path));
    assert!(remote::file_exists(&mut connection, &destination_path));

    remote::delete_file(&mut connection);
}

#[test]
fn download_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file5.txt");
    input.set_path(&path);

    let mut connection = Connection::to_container(Box::new(input));

    remote::create_file(&mut connection);

    remote::download_file(&mut connection);
    assert!(local::file_exists(&mut connection, &path));

    local::delete_file(&mut connection);
    remote::delete_file(&mut connection);
}

#[test]
fn upload_file() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file6.txt");
    input.set_path(&path);
	
	//going through this to make a local file
	let mut connection = Connection::to_container(Box::new(input));
    remote::create_file(&mut connection);
    remote::download_file(&mut connection);
    remote::delete_file(&mut connection);
	
	remote::put_file(&mut connection);
    local::delete_file(&mut connection);
	assert!(remote::file_exists(&mut connection, &path));
    remote::delete_file(&mut connection);
}