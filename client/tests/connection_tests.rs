extern crate client;

use client::connection::Connection;


#[test]
    let mut contents = Connection::read_log();
    contents.pop();

    if '\n' == contents.to_string().chars().last().unwrap()
    {
        contents.pop();
    }
    
    println!("{}", contents);
    assert!(contents == "Hostname: server:22 Username: root");
}
