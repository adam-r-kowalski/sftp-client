use ssh2::{Session, Sftp};
use std::fs::{File, OpenOptions};
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

fn open_log_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("connection.txt")
        .unwrap()
}

impl Connection {
    pub fn new(host: &str, username: &str, password: &str, input: Box<Input>) -> Connection {
        let tcp = TcpStream::connect(host).unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();
        session.userauth_password(username, password).unwrap();

        let mut log = open_log_file();

        if let Err(e) = writeln!(log, "Hostname: {} Username: {}", host, username) {
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

    pub fn read_log() -> String {
        let mut log = open_log_file();
        let mut info = String::new();
        log.read_to_string(&mut info).expect("something broke");
        info
    }
    pub fn view_connection_info(_connection: &mut Connection) -> String {
        Connection::read_log()
    }

    pub fn use_saved_connection(input: Box<Input>) -> Connection {
        let contents = Connection::read_log();
        let chunks: Vec<&str> = contents.split(" ").collect();
        let hostname = chunks[1];
        let mut username = chunks[3].to_string();
        username.pop();
        if '\n' == username.chars().last().unwrap() {
            username.pop();
        }

        Connection::new(&hostname, &username, "root", input)
    }
}
