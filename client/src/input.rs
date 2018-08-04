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

pub struct MockInput {
    mock_string: String,
    mock_prompt_path: PathBuf,
    mock_path: PathBuf,
}

impl MockInput {
    pub fn new() -> MockInput {
        MockInput {
            mock_string: String::new(),
            mock_prompt_path: PathBuf::from(String::new()),
            mock_path: PathBuf::from(String::new()),
        }
    }
}

impl MockInput {
    pub fn set_string(&mut self, value: String) {
        self.mock_string = value
    }

    pub fn set_prompt_path(&mut self, value: String) {
        self.mock_prompt_path = PathBuf::from(value)
    }

    pub fn set_path(&mut self, value: String) {
        self.mock_path = PathBuf::from(value)
    }
}

impl Input for MockInput {
    fn string(&self, _: &str) -> String {
        self.mock_string.clone()
    }

    fn prompt_path(&self, _: &str) -> PathBuf {
        self.mock_prompt_path.clone()
    }

    fn path(&self) -> PathBuf {
        self.mock_path.clone()
    }

    fn positive(&self, _: &str) -> usize {
        0
    }

    fn password(&self) -> String {
        String::new()
    }
}
