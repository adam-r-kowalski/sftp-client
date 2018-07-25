use ssh2::{Session, Sftp};
use std::net::TcpStream;

use input;

pub struct Connection {
    pub tcp: TcpStream,
    pub session: Session,
}

impl Connection {
    pub fn new(host: &str, username: &str, password: &str) -> Connection {
        let tcp = TcpStream::connect(host).unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();
        session.userauth_password(username, password).unwrap();

        Connection { tcp, session }
    }

    pub fn to_container() -> Connection {
        Connection::new("server:22", "root", "root")
    }

    pub fn from_prompt() -> Connection {
        let host = input::string("Enter host: ");
        let username = input::string("Enter username: ");
        let password = input::password();
        Connection::new(&host, &username, &password)
    }

    pub fn sftp(&self) -> Sftp {
        self.session.sftp().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn log_into_remote_ftp_server() {
        Connection::to_container();
    }

    #[test]
    fn create_directory_on_remote_server() {
        let connection = Connection::to_container();
        let sftp = connection.sftp();
        let path = Path::new("/demo");
        sftp.mkdir(path, 0).unwrap();
        sftp.readdir(path).unwrap();
        sftp.rmdir(path).unwrap();
    }
}
