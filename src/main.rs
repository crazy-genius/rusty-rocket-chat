extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::rc::Rc;

use gio::prelude::*;
use gtk::{Application, Button, Window};
use gtk::prelude::*;

use rocket_chat_lite::{rocket::authenticate};

fn on_activate(application: &gtk::Application) {
    let glide_src = include_str!("GUI.xml");
    let builder = gtk::Builder::from_string(glide_src);

    let main_window: Window = builder.get_object("main_window")
        .expect("couldn't load GUI");
    main_window.set_application(Some(application));

    main_window.show_all();
}

fn main() {
    let application = Application::new(
        Some("info.minkin.rocket_chat_lite"),
        Default::default(),
    ).expect("filed to initialize GTK application");

    application.connect_activate(on_activate);
    application.run(&std::env::args().collect::<Vec<_>>());
}
