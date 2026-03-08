use gtk4::prelude::*;
use gtk4::{ListBox, ListBoxRow, Label};

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
