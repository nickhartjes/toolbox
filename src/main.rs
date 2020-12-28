mod password_generator;

extern crate gtk;
extern crate gio;

use gtk::prelude::BuilderExtManual;
use gtk::{WidgetExt, GtkMenuItemExt, MenuShellExt, GtkWindowExt};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::path::Path;
use gdk_pixbuf::{Pixbuf, PixbufLoaderExt};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("gui/main.glade");
    let builder = gtk::Builder::from_string(glade_src);

    password_generator::password_generator::load(&builder);

    let window: gtk::Window = builder.get_object("window").unwrap();
    window.set_icon_from_file("./src/img/icons/128x128.png");
    window.show_all();


    // System tray app indicator
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("img").join("icons");
    println!("{:?}", icon_path);
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("64x64", "icon");
    let mut m = gtk::Menu::new();
    let mi = gtk::CheckMenuItem::with_label("Hello RUST");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);
    indicator.set_menu(&mut m);
    m.show_all();


    gtk::main();
}
