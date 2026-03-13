use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ListBox, ListBoxRow, Label};

pub fn build_sidebar() -> ListBox {
    let list = ListBox::new();
    let tables = ["inet filter", "ip nat"];
    for table in tables {
        let row = ListBoxRow::new();
        row.set_child(Some(&Label::new(Some(table))));
        list.append(&row);
    }
    list
}
