use connection::Connection;
use input;
use std::io::Read;

pub fn list_directories(connection: &Connection) {
    connection.sftp().readdir(&input::path())
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.0));
}

pub fn create_directory(connection: &Connection) {
    connection.sftp().mkdir(&input::path(), 0).unwrap()
}

pub fn delete_directory(connection: &Connection) {
    connection.sftp().rmdir(&input::path()).unwrap()
}

pub fn rename_file(connection: &Connection) {
    let source = &input::prompt_path("\nEnter source: ");
    let destination = &input::prompt_path("\nEnter destination: ");
    connection.sftp().rename(source, destination, None).unwrap();
}

pub fn download_file(connection: &Connection){
	let target = &input::prompt_path("\nWhich file would you like to download?: ");
	let (mut remote_file,stat) = connection.session.scp_recv(target).unwrap();
	let mut contents = Vec::new();
	remote_file.read_to_end(&mut contents).unwrap();
	std::fs::write("download.txt",contents).unwrap();	
}