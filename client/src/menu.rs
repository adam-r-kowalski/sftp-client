use std::{fmt, io};

pub struct MenuItem<Context> {
    title: String,
    callback: Box<Fn(&Context)>
}

impl<Context> fmt::Display for MenuItem<Context> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)
    }
}

impl<Context> MenuItem<Context> {
    pub fn new(title: &str, callback: Box<Fn(&Context) -> ()>) -> MenuItem<Context> {
        MenuItem {
            title: String::from(title),
            callback
        }
    }

    pub fn call(&self, context: &Context) {
        (self.callback)(&context)
    }
}

pub struct Menu<Context> {
    context: Context,
    name: String,
    menu_items: Vec<MenuItem<Context>>,
}

impl<Context> Menu<Context> {
    pub fn new(name: &str, context: Context) -> Menu<Context> {
        Menu {
            context,
            name: String::from(name),
            menu_items: vec![]
        }
    }

    pub fn push(&mut self, menu_item: MenuItem<Context>) {
        self.menu_items.push(menu_item)
    }

    pub fn get_input(&self) {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let trimmed = input_text.trim();
        let index = trimmed.parse::<usize>().unwrap();
        self.menu_items[index].call(&self.context);
    }
}

impl<Context> fmt::Display for Menu<Context> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = writeln!(f, "--- {} ---", self.name);

        for (index, menu_item) in (&self.menu_items).into_iter().enumerate() {
            write!(f, "{} - {}", index, menu_item)?;
        }

        result
    }
}
