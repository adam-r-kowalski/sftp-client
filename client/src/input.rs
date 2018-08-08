use rpassword;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub trait Input {
    fn string(&mut self, prompt: &str) -> String;
    fn prompt_path(&mut self, prompt: &str) -> PathBuf;
    fn path(&mut self) -> PathBuf;
    fn positive(&mut self, prompt: &str) -> usize;
    fn password(&mut self) -> String;
}

pub struct ConsoleInput {}

impl ConsoleInput {
    pub fn new() -> ConsoleInput {
        ConsoleInput {}
    }
}

impl Input for ConsoleInput {
    fn string(&mut self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        String::from(input_text.trim())
    }

    fn prompt_path(&mut self, prompt: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.string(prompt));
        path
    }

    fn path(&mut self) -> PathBuf {
        self.prompt_path("\nEnter path: ")
    }

    fn positive(&mut self, prompt: &str) -> usize {
        self.string(prompt).parse::<usize>().unwrap()
    }

    fn password(&mut self) -> String {
        rpassword::prompt_password_stdout("Enter password: ").unwrap()
    }
}

pub struct MockInput {
    mock_string: String,
    mock_strings: VecDeque<String>,
    mock_prompt_path: PathBuf,
    mock_prompt_paths: VecDeque<PathBuf>,
    mock_path: PathBuf,
    mock_paths: VecDeque<PathBuf>,
}

/* 
  This is used to replace console input for tests.
  Normally the user is prompted for input and we needed
  to abstract that away so the a user is not necessary at
  test time
*/
impl MockInput {
    pub fn new() -> MockInput {
        MockInput {
            mock_string: String::new(),
            mock_strings: VecDeque::new(),
            mock_prompt_path: PathBuf::new(),
            mock_prompt_paths: VecDeque::new(),
            mock_path: PathBuf::new(),
            mock_paths: VecDeque::new(),
        }
    }

    pub fn set_string(&mut self, value: &str) {
        self.mock_string = String::from(value)
    }

    pub fn set_strings(&mut self, values: &[&str]) {
        for value in values {
            self.mock_strings.push_back(value.to_string());
        }
    }

    pub fn set_prompt_path(&mut self, value: &Path) {
        self.mock_prompt_path = PathBuf::from(value)
    }

    pub fn set_prompt_paths(&mut self, values: &[&Path]) {
        for value in values {
            self.mock_prompt_paths.push_back(PathBuf::from(value));
        }
    }

    pub fn set_path(&mut self, value: &Path) {
        self.mock_path = PathBuf::from(value)
    }

    pub fn set_paths(&mut self, values: &[&Path]) {
        for value in values {
            self.mock_paths.push_back(PathBuf::from(value));
        }
    }
}

impl Input for MockInput {
    fn string(&mut self, _: &str) -> String {
        match self.mock_strings.pop_front() {
            Some(p) => p.clone(),
            None => self.mock_string.clone(),
        }
    }

    fn prompt_path(&mut self, _: &str) -> PathBuf {
        match self.mock_prompt_paths.pop_front() {
            Some(p) => p.clone(),
            None => self.mock_prompt_path.clone(),
        }
    }

    fn path(&mut self) -> PathBuf {
        match self.mock_paths.pop_front() {
            Some(p) => p.clone(),
            None => self.mock_path.clone(),
        }
    }

    fn positive(&mut self, _: &str) -> usize {
        0
    }

    fn password(&mut self) -> String {
        String::new()
    }
}
