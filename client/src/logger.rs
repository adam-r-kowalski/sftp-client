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
