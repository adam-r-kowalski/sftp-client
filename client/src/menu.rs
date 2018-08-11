use std::fmt;

use connection::Connection;
use logger::log;

pub struct MenuItem {
    title: String,
    callback: fn(&mut Connection) -> String,
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)
    }
}

impl MenuItem {
    pub fn new(title: &str, callback: fn(&mut Connection) -> String) -> MenuItem {
        MenuItem {
            title: String::from(title),
            callback,
        }
    }

    pub fn select(&self, connection: &mut Connection) -> String {
        (self.callback)(connection)
    }
}

pub struct Menu {
    connection: Connection,
    name: String,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(name: &str, connection: Connection) -> Menu {
        Menu {
            connection,
            name: String::from(name),
            menu_items: vec![],
        }
    }

    pub fn insert(&mut self, menu_item: MenuItem) {
        self.menu_items.push(menu_item)
    }

    pub fn show(&mut self) {
        print!("{}", self);
        let index = self.connection.input.positive("Enter choice: ");
        let log_entry = self.menu_items[index].select(&mut self.connection);
        log(&log_entry);
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = writeln!(f, "--- {} ---", self.name);

        for (index, menu_item) in (&self.menu_items).into_iter().enumerate() {
            write!(f, "{} - {}", index, menu_item)?;
        }

        result
    }
}
