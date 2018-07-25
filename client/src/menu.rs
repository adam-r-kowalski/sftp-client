use std::fmt;
use std::marker::PhantomData;

use input;
use logger;

pub struct MenuItem<Context> {
    title: String,
    callback: fn(&Context) -> String,
}

impl<Context> fmt::Display for MenuItem<Context> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)
    }
}

impl<Context> MenuItem<Context> {
    pub fn new(title: &str, callback: fn(&Context) -> String) -> MenuItem<Context> {
        MenuItem {
            title: String::from(title),
            callback,
        }
    }
}

impl<'a, Context> Fn<(&'a Context,)> for MenuItem<Context> {
    extern "rust-call" fn call(&self, args: (&'a Context,)) -> Self::Output {
        (self.callback)(args.0)
    }
}

impl<'a, Context> FnMut<(&'a Context,)> for MenuItem<Context> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a Context,)) -> Self::Output {
        self.call(args)
    }
}

impl<'a, Context> FnOnce<(&'a Context,)> for MenuItem<Context> {
    type Output = String;

    extern "rust-call" fn call_once(self, args: (&'a Context,)) -> Self::Output {
        self.call(args)
    }
}

pub struct Menu<Logger: logger::Logger, Context> {
    context: Context,
    name: String,
    menu_items: Vec<MenuItem<Context>>,
    logger: PhantomData<Logger>,
}

impl<Logger: logger::Logger, Context> Menu<Logger, Context> {
    pub fn new(name: &str, context: Context) -> Menu<Logger, Context> {
        Menu {
            context,
            name: String::from(name),
            menu_items: vec![],
            logger: PhantomData,
        }
    }

    pub fn insert(&mut self, menu_item: MenuItem<Context>) {
        self.menu_items.push(menu_item)
    }
}

impl<Logger: logger::Logger, Context> Fn<()> for Menu<Logger, Context> {
    extern "rust-call" fn call(&self, _: ()) {
        print!("{}", self);
        let index = input::positive("Enter choice: ");
        Logger::log(&self.menu_items[index](&self.context));
    }
}

impl<Logger: logger::Logger, Context> FnMut<()> for Menu<Logger, Context> {
    extern "rust-call" fn call_mut(&mut self, args: ()) {
        self.call(args)
    }
}

impl<Logger: logger::Logger, Context> FnOnce<()> for Menu<Logger, Context> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: ()) {
        self.call(args)
    }
}

impl<Logger: logger::Logger, Context> fmt::Display for Menu<Logger, Context> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = writeln!(f, "--- {} ---", self.name);

        for (index, menu_item) in (&self.menu_items).into_iter().enumerate() {
            write!(f, "{} - {}", index, menu_item)?;
        }

        result
    }
}
