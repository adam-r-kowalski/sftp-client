#![feature(unboxed_closures, fn_traits)]

extern crate ssh2;

pub mod menu;
pub mod remote;

use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn log_into_remote_ftp_server() {
    let tcp = TcpStream::connect("server:22").unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_password("root", "root").unwrap();
  }

  #[test]
    fn create_directory_on_remote_server() {
        let tcp = TcpStream::connect("server:22").unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();
        session.userauth_password("root", "root").unwrap();

        let sftp = session.sftp().unwrap();

        let path = Path::new("/demo");

        sftp.mkdir(path, 0).unwrap();
        sftp.readdir(path).unwrap();
        sftp.rmdir(path).unwrap();
    }
}
