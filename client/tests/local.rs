extern crate client;

use std::path::PathBuf;

use std::fs::File;
use client::connection::Connection;
use client::input::MockInput;
use client::local;


#[test]
fn change_permissions() {
    let mut input = MockInput::new();
    let path = PathBuf::from("/demo_file20.txt");
    input.set_path(&path);
    input.set_string("777");

    let mut connection = Connection::to_container(Box::new(input));

    let f = File::create("/demo_file20.txt").unwrap();
    let metadata = f.metadata().unwrap();
    assert_eq!(false, metadata.permissions().readonly());
    local::change_permission(&mut connection);
    assert_eq!(false, metadata.permissions().readonly());
    local::delete_file(&mut connection);
}
