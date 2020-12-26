extern crate gtk;
extern crate gio;

use gtk::prelude::BuilderExtManual;
use gtk::{WidgetExt};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").unwrap();

    window.show_all();

    gtk::main();
}
