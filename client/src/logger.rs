/*
  Every action will have a corresponding log entry
  so that the user can keep track of the actions that
  they have taken. It will also be useful for diagnostics
  to find out why something went wrong
*/

extern crate chrono;
use self::chrono::Local;

use std::fs::OpenOptions;
use std::io::Write;

pub trait Logger {
    fn log(&self, content: &str);
}

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger{}
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, content: &str) {

        println!("{}", content);

        let mut record = OpenOptions::new()
            .write(true)
	    .append(true)
            .create(true)
            .open("logger.txt")
            .unwrap();

        let date = Local::now();
       

        if let Err(e) = writeln!(record, "{} {}",date.format("[%Y-%m-%d][%H:%M:%S]"),content) {
            eprintln!("Couldn't write to logger: {}", e);
        }

       //println!("{}", content)

    }

}

