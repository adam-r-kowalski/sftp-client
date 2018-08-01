use connection::Connection;
use input;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn list_directories(connection: &Connection) -> String {
    let path = input::path();
    connection
        .sftp()
        .readdir(&path)
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.0));
    format!("User listed remote directories at {:?}", path)
}

pub fn create_directory(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().mkdir(&path, 0).unwrap();
    format!("User created remote directory {:?}", path)
}

pub fn delete_directory(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().rmdir(&path).unwrap();
    format!("User deleted remote directory {:?}", path)
}

pub fn rename_file(connection: &Connection) -> String {
    let source = &input::prompt_path("\nEnter source: ");
    let destination = &input::prompt_path("\nEnter destination: ");
    connection.sftp().rename(source, destination, None).unwrap();
    format!("User renamed remote file {:?} to {:?}", source, destination)
}

pub fn create_file(connection: &Connection) -> String {
    let sftp = connection.sftp();
    let path = input::path();
    sftp.create(&path).unwrap();
    format!("User created remote file {:?}", path)
}

pub fn delete_file(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().unlink(&path).unwrap();
    format!("User deleted remote file {:?}", path)
}

pub fn put_file(connection: &Connection) -> String {
    let source = input::prompt_path("\nLocal path to upload: ");
    let dest = input::prompt_path("\nRemote destination path: ");
    let mut f = File::open(source).unwrap();
    let mut contents = Vec::new();

    f.read_to_end(&mut contents).unwrap();
    let mut remote_file = connection
        .session
        .scp_send(Path::new(&dest), 0o644, contents.len() as u64, None)
        .unwrap();
    remote_file.write(&contents).unwrap();

    format!("User uploaded a file to remote server: {:?}", dest)
}

pub fn put_file_multi(connection: &Connection) -> String {
    let mut done = false;

    while !done {
        let source = input::prompt_path("\nLocal path to upload: ");
        let dest = input::prompt_path("\nRemote destination path: ");

        let mut f = File::open(source).unwrap();
        let mut contents = Vec::new();

        f.read_to_end(&mut contents).unwrap();
        let mut remote_file = connection
            .session
            .scp_send(Path::new(&dest), 0o644, contents.len() as u64, None)
            .unwrap();
        remote_file.write(&contents).unwrap();

        let response = &input::string("\nAnother file?(yes or no)");
        if response == "no" {
            done = true;
        }
    }
    format!("User uploaded multiple files to remote server.")
}

pub fn download_file(connection: &Connection) -> String {
    let target = &input::prompt_path("\nWhich file would you like to download?: ");
    let (mut remote_file, _stat) = connection.session.scp_recv(target).unwrap();
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();
    std::fs::write(target, contents).unwrap();
    format!("User downloaded a remote file {:?}", target)
}

pub fn download_file_multi(connection: &Connection) -> String {
    let mut done = false;

    while !done {
        let target = &input::prompt_path("\nWhich file would you like to download?: ");
        let (mut remote_file, _stat) = connection.session.scp_recv(target).unwrap();
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();
        std::fs::write(target, contents).unwrap();
        let response = &input::string("\nContinue?(yes or no)");
        if response == "no" {
            done = true;
        }
    }
    format!("User downloaded multiple files")
}

pub fn change_permission(connection: &Connection) -> String {
    let path = input::path();
    let permissions = input::string("\nEnter new file permissions: ");
    let command = format!("chmod {} {}", permissions, path.to_str().unwrap());
    connection.remote_execute(&command);
    format!(
        "Changed permissions for remote file {:?} to {}",
        path, permissions
    )
}

pub fn execute(connection: &Connection) -> String {
    connection.remote_execute(&input::string("\nEnter command: "))
}
