use gtk4 as gtk;
use gtk::prelude::*;

pub fn build_rules_table() -> gtk::Box {

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let reload_btn = gtk::Button::with_label("Reload");

    container.append(&reload_btn);

    container
}