use gtk4 as gtk;
use gtk::prelude::*;

pub fn open_rule_editor(parent: &gtk::ApplicationWindow) {

    let dialog = gtk::Dialog::builder()
        .transient_for(parent)
        .modal(true)
        .title("Add Rule")
        .default_width(400)
        .default_height(200)
        .build();

    let content = dialog.content_area();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);
    container.set_margin_start(10);
    container.set_margin_end(10);

    let add_btn = gtk::Button::with_label("Add rule");

    add_btn.connect_clicked(|_| {
        println!("Add rule clicked");
    });

    container.append(&add_btn);
    content.append(&container);

    dialog.present();
}