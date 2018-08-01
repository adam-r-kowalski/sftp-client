use connection::Connection;
use input;
use std::fs;
use std::fs::File;
use std::process::Command;
use std::str::from_utf8;

pub fn list_directories(_: &Connection) -> String {
    let path = input::path();
    match fs::read_dir(&path) {
        Ok(p) => {
            p.into_iter()
                .for_each(|d| println!("{:?}", d.unwrap().path()));
            format!("User listed local directories at {:?}", path)
        }
        Err(e) => e.to_string(),
    }
}

pub fn rename_file(_: &Connection) -> String {
    let old_path = input::prompt_path("Enter old path: ");
    let new_path = input::prompt_path("Enter new path: ");
    match fs::rename(&old_path, &new_path) {
        Ok(_) => format!("Renamed {:?} to {:?}.", old_path, new_path),
        Err(e) => e.to_string(),
    }
}

pub fn change_permission(_: &Connection) -> String {
    let path = input::path();
    match File::open(&path) {
        Ok(file) => {
            let mut perms = file.metadata().unwrap().permissions();
            perms.set_readonly(true);
            file.set_permissions(perms).unwrap();
            format!("Changed permission for local file {:?} ", path)
        }
        Err(e) => e.to_string(),
    }
}

fn execute_command(command: &str) -> String {
    let split = command.split(" ").collect::<Vec<_>>();
    let mut command_builder = Command::new(split[0]);
    command_builder.args(split[1..].iter());
    let output = command_builder.output().unwrap().stdout;
    String::from(from_utf8(&output).unwrap())
}

pub fn execute(_: &Connection) -> String {
    format!("{}", execute_command(&input::string("\nEnter command: ")))
}
