use connection::Connection;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn list_directories(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match connection.sftp().readdir(&path) {
        Ok(d) => {
            d.into_iter().for_each(|d| println!("{:?}", d.0));
            format!("User listed remote directories at {:?}", path)
        }
        Err(e) => e.to_string(),
    }
}

pub fn directory_exists(connection: &mut Connection, path: &Path) -> bool {
    connection.sftp().readdir(path).is_ok()
}

pub fn create_directory(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match connection.sftp().mkdir(&path, 0) {
        Ok(_) => format!("User created remote directory {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn delete_directory(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match connection.sftp().rmdir(&path) {
        Ok(_) => format!("User deleted remote directory {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn rename_file(connection: &mut Connection) -> String {
    let source = connection.input.prompt_path("\nEnter source: ");
    let destination = connection.input.prompt_path("\nEnter destination: ");
    match connection.sftp().rename(&source, &destination, None) {
        Ok(_) => format!("User renamed remote file {:?} to {:?}", source, destination),
        Err(e) => e.to_string(),
    }
}

pub fn file_exists(connection: &mut Connection, path: &Path) -> bool {
    let sftp = connection.sftp();
    return sftp.open(path).is_ok();
}

pub fn create_file(connection: &mut Connection) -> String {
    let path = connection.input.path();
    let sftp = connection.sftp();
    let result = sftp.create(&path);
    match result {
        Ok(_) => format!("User created remote file {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn delete_file(connection: &mut Connection) -> String {
    let path = connection.input.path();
    match connection.sftp().unlink(&path) {
        Ok(_) => format!("User deleted remote file {:?}", path),
        Err(e) => e.to_string(),
    }
}

pub fn put_file(connection: &mut Connection) -> String {
    let source = connection.input.prompt_path("\nLocal path to upload: ");

    match File::open(source) {
        Ok(mut f) => {
            let mut contents = Vec::new();
            f.read_to_end(&mut contents).unwrap();

            let dest = connection.input.prompt_path("\nRemote destination path: ");
            connection
                .session
                .scp_send(Path::new(&dest), 0o644, contents.len() as u64, None)
                .map(|mut remote_file| remote_file.write(&contents))
                .map(|_| format!("User uploaded a file to remote server: {:?}", dest))
                .unwrap_or_else(|e| e.to_string())
        }
        Err(e) => e.to_string(),
    }
}

pub fn put_file_multi(connection: &mut Connection) -> String {
    let mut done = false;

    while !done {
        put_file(connection);
        let response = connection.input.string("\nAnother file?(yes or no)");
        done = response == "no";
    }
    format!("User uploaded multiple files to remote server.")
}

pub fn download_file(connection: &mut Connection) -> String {
    let target = connection
        .input
        .prompt_path("\nWhich file would you like to download?: ");

    match connection.session.scp_recv(&target) {
        Ok((mut remote_file, _)) => {
            let mut contents = Vec::new();
            remote_file.read_to_end(&mut contents).unwrap();
            std::fs::write(&target, contents).unwrap();
            format!("User downloaded a remote file {:?}", target)
        }
        Err(e) => e.to_string(),
    }
}

pub fn download_file_multi(connection: &mut Connection) -> String {
    let mut done = false;

    while !done {
        download_file(connection);
        let response = connection.input.string("\nContinue?(yes or no)");
        done = response == "no"
    }
    format!("User downloaded multiple files")
}

pub fn file_permissions(connection: &mut Connection, path: &Path) -> u32 {
    let sftp = connection.sftp();
    let file_stat = sftp.stat(path).unwrap();
    file_stat.perm.unwrap()
}

pub fn change_permission(connection: &mut Connection) -> String {
    let path = connection.input.path();
    let permissions = connection.input.string("\nEnter new file permissions: ");
    let command = format!("chmod {} {}", permissions, path.to_str().unwrap());
    connection.remote_execute(&command);
    format!(
        "Changed permissions for remote file {:?} to {}",
        path, permissions
    )
}

pub fn execute(connection: &mut Connection) -> String {
    let command = connection.input.string("\nEnter command: ");
    connection.remote_execute(&command)
}
