use connection::Connection;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

pub fn list_directories(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match fs::read_dir(&path) {
        Ok(p) => {
            p.into_iter()
                .for_each(|d| println!("{:?}", d.unwrap().path()));
            format!("User listed local directories at {:?}", path)
        }
        Err(e) => e.to_string(),
    }
}

pub fn rename_file(connection: &mut Connection) -> String {
    let old_path = connection.input.prompt_path("Enter old path: ");
    let new_path = connection.input.prompt_path("Enter new path: ");
    match fs::rename(&old_path, &new_path) {
        Ok(_) => format!("Renamed {:?} to {:?}.", old_path, new_path),
        Err(e) => e.to_string(),
    }
}

pub fn change_permission(connection: &mut Connection) -> String {
    let path = connection.input.path();
    let permissions = connection
        .input
        .string("Enter new permissions in octal format (e.g. 754): ");
    let command = format!("chmod {} {}", permissions, path.to_str().unwrap());
    execute_command(&command);
    format!(
        "Changed permissions for local file {:?} to {}",
        path, permissions
    )
}

pub fn execute_command(command: &str) -> String {
    let split = command.split(" ").collect::<Vec<_>>();
    let mut command_builder = Command::new(split[0]);
    command_builder.args(split[1..].iter());
    let output = command_builder.output().unwrap().stdout;
    String::from(from_utf8(&output).unwrap())
}

pub fn create_file(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match File::create(&path) {
        Ok(_) => format!("User created local file {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn delete_file(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match fs::remove_file(&path) {
        Ok(_) => format!("User deleted local file {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn file_exists(_: &mut Connection, path: &Path) -> bool {
    File::open(&path).is_ok()
}
