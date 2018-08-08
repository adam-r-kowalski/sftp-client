/*
  Every action will have a corresponding log entry
  so that the user can keep track of the actions that
  they have taken. It will also be useful for diagnostics
  to find out why something went wrong
*/

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
        println!("{}", content)
    }
}
