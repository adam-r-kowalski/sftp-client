use std::fmt;

use input;
use logger::Logger;
use connection::Connection;

pub struct MenuItem {
    title: String,
    callback: fn(&Connection) -> String,
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)
    }
}

impl MenuItem {
    pub fn new(title: &str, callback: fn(&Connection) -> String) -> MenuItem {
        MenuItem {
            title: String::from(title),
            callback,
        }
    }
}

impl<'a> Fn<(&'a Connection,)> for MenuItem {
    extern "rust-call" fn call(&self, args: (&'a Connection,)) -> Self::Output {
        (self.callback)(&args.0)
    }
}

impl<'a> FnMut<(&'a Connection,)> for MenuItem {
    extern "rust-call" fn call_mut(&mut self, args: (&'a Connection,)) -> Self::Output {
        self.call(args)
    }
}

impl<'a> FnOnce<(&'a Connection,)> for MenuItem {
    type Output = String;

    extern "rust-call" fn call_once(self, args: (&'a Connection,)) -> Self::Output {
        self.call(args)
    }
}

pub struct Menu {
    connection: Connection,
    name: String,
    menu_items: Vec<MenuItem>,
    logger: Box<Logger>,
}

impl Menu {
    pub fn new(name: &str, connection: Connection, logger: Box<Logger>) -> Menu {
        Menu {
            connection,
            name: String::from(name),
            menu_items: vec![],
            logger: logger,
        }
    }

    pub fn insert(&mut self, menu_item: MenuItem) {
        self.menu_items.push(menu_item)
    }
}

impl Fn<()> for Menu {
    extern "rust-call" fn call(&self, _: ()) {
        print!("{}", self);
        let index = input::positive("Enter choice: ");
        self.logger.log(&self.menu_items[index](&self.connection));
    }
}

impl FnMut<()> for Menu {
    extern "rust-call" fn call_mut(&mut self, args: ()) {
        self.call(args)
    }
}

impl FnOnce<()> for Menu {
    type Output = ();

    extern "rust-call" fn call_once(self, args: ()) {
        self.call(args)
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
