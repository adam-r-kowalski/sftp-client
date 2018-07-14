use ssh2::Sftp;
use std::path::PathBuf;
use std::io;
use std::io::Write;

fn get_path_buf() -> PathBuf {
    print!("\nEnter path: ");
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    let mut path = PathBuf::new();
    path.push(trimmed);
    path
}

pub fn create_directory(sftp: &Sftp) {
    sftp.mkdir(get_path_buf().as_path(), 0).unwrap()
}

pub fn delete_directory(sftp: &Sftp) {
    sftp.rmdir(get_path_buf().as_path()).unwrap()
}

pub fn list_directories(sftp: &Sftp) {
    sftp.readdir(get_path_buf().as_path())
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.0));
}

pub fn rename_file(sftp: &Sftp) {
    let source = get_path_buf();
    let destination = get_path_buf();
    sftp.rename(&source, &destination, None).unwrap();
}
