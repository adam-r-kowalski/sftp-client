extern crate chrono;
use self::chrono::Local;

use std::fs::{File, OpenOptions};
use std::io::Write;

/*
  Every action will have a corresponding log entry
  so that the user can keep track of the actions that
  they have taken. It will also be useful for diagnostics
  to find out why something went wrong
*/
pub struct Logger {
    log_file: File,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            log_file: OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("logger.txt")
                .unwrap(),
        }
    }

    pub fn log(&mut self, content: &str) {
        println!("{}", content);

        let formated_date = Local::now().format("[%Y-%m-%d][%H:%M:%S]");
        if let Err(e) = writeln!(self.log_file, "{} {}", formated_date, content) {
            eprintln!("Couldn't write to logger: {}", e);
        }
    }
}
