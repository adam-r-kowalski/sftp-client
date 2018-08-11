extern crate client;

use client::connection::Connection;
use client::input::MockInput;

#[test]
fn connection_log() {
    let _ = Connection::to_container(MockInput::new());
    let mut contents = Connection::read_log();
    contents.pop();

    if '\n' == contents.to_string().chars().last().unwrap() {
        contents.pop();
    }

    println!("{}", contents);
    assert!(contents == "Hostname: server:22 Username: root");
}
