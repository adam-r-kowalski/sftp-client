use ssh2::{Session, Sftp};
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use input::Input;

// This struct is used to establish a connection to a sftp server
pub struct Connection {
    pub tcp: TcpStream,
    pub session: Session,
    pub input: Box<Input>,
}

impl Connection {
    pub fn new(host: &str, username: &str, password: &str, input: Box<Input>) -> Connection {
        let tcp = TcpStream::connect(host).unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();
        session.userauth_password(username, password).unwrap();

        let mut log = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("connection.txt")
            .unwrap();

        if let Err(e) = writeln!(log, "Hostname: {}  Username: {}", host, username) {
            eprintln!("Couldn't write to log: {}", e);
        }
        Connection {
            tcp,
            session,
            input,
        }
    }

    pub fn to_container(input: Box<Input>) -> Connection {
        Connection::new("server:22", "root", "root", input)
    }

    pub fn from_prompt(mut input: Box<Input>) -> Connection {
        let host = input.string("Enter host: ");
        let username = input.string("Enter username: ");
        let password = input.password();
        Connection::new(&host, &username, &password, input)
    }

    pub fn sftp(&self) -> Sftp {
        self.session.sftp().unwrap()
    }

    pub fn remote_execute(&self, command: &str) -> String {
        let mut channel = self.session.channel_session().unwrap();
        channel.exec(command).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close().unwrap();
        s
    }

    pub fn read_log(&self) -> String {
        let mut log = OpenOptions::new()
            .read(true)
            .open("connection.txt")
            .unwrap();

        let mut info = String::new();
        log.read_to_string(&mut info).expect("something broke");

        info
    }
    pub fn view_connection_info(connection: &mut Connection) -> String {
        connection.read_log()
    }
}
