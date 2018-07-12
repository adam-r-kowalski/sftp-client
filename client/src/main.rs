extern crate client;
extern crate ssh2;

use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;

fn main() {
    let tcp = TcpStream::connect("server:22").unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_password("root", "root").unwrap();

    let sftp = session.sftp().unwrap();

    // sftp.mkdir(Path::new("/demo"), 0).unwrap();

    // sftp.rmdir(Path::new("/demo")).unwrap();

    sftp.readdir(Path::new("/"))
        .unwrap()
        .into_iter()
        .for_each(|(path, _)| println!("{:?}", path));
}
