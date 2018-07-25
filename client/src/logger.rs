pub trait Logger {
    fn log(content: &str);
}

pub struct ConsoleLogger {}

impl Logger for ConsoleLogger {
    fn log(content: &str) {
        println!("{}", content)
    }
}
