extern crate chrono;
use self::chrono::Local;

use std::fs::OpenOptions;
use std::io::Write;

/*
  Every action will have a corresponding log entry
  so that the user can keep track of the actions that
  they have taken. It will also be useful for diagnostics
  to find out why something went wrong
*/
pub fn log(content: &str) {
    println!("{}", content);

    let mut record = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("logger.txt")
        .unwrap();

    let formated_date = Local::now().format("[%Y-%m-%d][%H:%M:%S]");
    if let Err(e) = writeln!(record, "{} {}", formated_date, content) {
        eprintln!("Couldn't write to logger: {}", e);
    }
}
