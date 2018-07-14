use std::fmt;

use input;

pub struct MenuItem<Context> {
    title: String,
    callback: fn(&Context)
}

impl<Context> fmt::Display for MenuItem<Context> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)
    }
}

impl<Context> MenuItem<Context> {
    pub fn new(title: &str, callback: fn(&Context)) -> MenuItem<Context> {
        MenuItem {
            title: String::from(title),
            callback
        }
    }
}

impl<'a, Context> Fn<(&'a Context,)> for MenuItem<Context> {
    extern "rust-call" fn call(&self, args: (&'a Context,)) {
        (self.callback)(args.0)
    }
}

impl<'a, Context> FnMut<(&'a Context,)> for MenuItem<Context> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a Context,)) {
        self.call(args)
    }
}

impl<'a, Context> FnOnce<(&'a Context,)> for MenuItem<Context> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (&'a Context,)) {
        self.call(args)
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

    pub fn insert(&mut self, menu_item: MenuItem<Context>) {
        self.menu_items.push(menu_item)
    }
}

impl<Context> Fn<()> for Menu<Context> {
    extern "rust-call" fn call(&self, _: ()) {
        print!("{}", self);
        let index = input::positive("Enter choice: ");
        self.menu_items[index](&self.context);
    }
}

impl<Context> FnMut<()> for Menu<Context> {
    extern "rust-call" fn call_mut(&mut self, args: ()) {
        self.call(args)
    }
}

impl<Context> FnOnce<()> for Menu<Context> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: ()) {
        self.call(args)
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
