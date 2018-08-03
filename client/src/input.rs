use rpassword;
use std::io::{self, Write};
use std::path::PathBuf;

pub trait Input {
    fn string(&self, prompt: &str) -> String;
    fn prompt_path(&self, prompt: &str) -> PathBuf;
    fn path(&self) -> PathBuf;
    fn positive(&self, prompt: &str) -> usize;
    fn password(&self) -> String;
}

pub struct ConsoleInput {}

impl ConsoleInput {
    pub fn new() -> ConsoleInput {
        ConsoleInput {}
    }
}

impl Input for ConsoleInput {
    fn string(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        String::from(input_text.trim())
    }

    fn prompt_path(&self, prompt: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.string(prompt));
        path
    }

    fn path(&self) -> PathBuf {
        self.prompt_path("\nEnter path: ")
    }

    fn positive(&self, prompt: &str) -> usize {
        self.string(prompt).parse::<usize>().unwrap()
    }

    fn password(&self) -> String {
        rpassword::prompt_password_stdout("Enter password: ").unwrap()
    }
}
