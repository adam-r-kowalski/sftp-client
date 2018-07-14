use connection::Connection;
use input;

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
