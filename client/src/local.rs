use std::fs;
use std::fs::File;
use connection::Connection;
use input;

pub fn list_directories(_: &Connection) -> String {
    let path = input::path();
    fs::read_dir(&path)
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.unwrap().path()));
    format!("User listed local directories at {:?}", path)
}

pub fn rename_file(_: &Connection) -> String {
    let old_path = input::prompt_path("Enter old path: ");
    let new_path = input::prompt_path("Enter new path: ");
    match fs::rename(&old_path, &new_path) {
        Ok(_)   => format!("Renamed {:?} to {:?}.", old_path, new_path),
        Err(e)  => e.to_string(),
    }
}

pub fn change_permission(_: &Connection) -> String {
    let path = input::path();
    match File::open(&path) {
        Ok(file)  => { let mut perms = file.metadata().unwrap().permissions();
                       perms.set_readonly(true);
                       file.set_permissions(perms).unwrap();
                       format!("Changed permisssions for local file {:?} ", path)
                     },
        Err(e)    => e.to_string(),
    }
}
