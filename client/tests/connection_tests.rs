extern crate client;

use client::connection::Connection;
use client::input::ConsoleInput;


#[test]
fn connection_log () {
 
    let connection = Connection::to_container(Box::new(ConsoleInput::new()));
    let mut contents = connection.read_log();
    contents.pop();
    
    println!("{}", contents);
    assert!(contents == "Hostname: server:22  Username: root");

   

}
