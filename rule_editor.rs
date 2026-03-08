use gtk4::prelude::*;
use gtk4::{Dialog, Entry, ComboBoxText, Button, Box, Orientation, Label};

use crate::nft;

pub fn open_rule_editor(parent: &gtk4::ApplicationWindow) {

    let dialog = Dialog::builder()
        .transient_for(parent)
        .title("Add Firewall Rule")
        .modal(true)
        .build();

    let content_area = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 5);

    let proto_box = Box::new(Orientation::Horizontal, 5);
    proto_box.append(&Label::new(Some("Protocol:")));
    let proto_combo = ComboBoxText::new();
    for p in &["tcp","udp","icmp","any"] {
        proto_combo.append_text(p);
    }
    proto_combo.set_active(Some(0));
    proto_box.append(&proto_combo);
    vbox.append(&proto_box);

    let src_box = Box::new(Orientation::Horizontal,5);
    src_box.append(&Label::new(Some("Source IP:")));
    let src_entry = Entry::new();
    src_entry.set_placeholder_text(Some("any"));
    src_box.append(&src_entry);
    vbox.append(&src_box);

    let dst_box = Box::new(Orientation::Horizontal,5);
    dst_box.append(&Label::new(Some("Destination IP:")));
    let dst_entry = Entry::new();
    dst_entry.set_placeholder_text(Some("any"));
    dst_box.append(&dst_entry);
    vbox.append(&dst_box);

    let port_box = Box::new(Orientation::Horizontal,5);
    port_box.append(&Label::new(Some("Port:")));
    let port_entry = Entry::new();
    port_entry.set_placeholder_text(Some("any"));
    port_box.append(&port_entry);
    vbox.append(&port_box);

    let action_box = Box::new(Orientation::Horizontal,5);
    action_box.append(&Label::new(Some("Action:")));
    let action_combo = ComboBoxText::new();
    for a in &["accept","drop","reject"] {
        action_combo.append_text(a);
    }
    action_combo.set_active(Some(0));
    action_box.append(&action_combo);
    vbox.append(&action_box);

    let add_btn = Button::with_label("Add Rule");
    vbox.append(&add_btn);

    content_area.append(&vbox);

    let pc = proto_combo.clone();
    let se = src_entry.clone();
    let de = dst_entry.clone();
    let pe = port_entry.clone();
    let ac = action_combo.clone();

    add_btn.connect_clicked(move |_| {
        let proto = pc.active_text().unwrap_or_else(|| "any".into());
        let src = se.text().to_string();
        let dst = de.text().to_string();
        let port = pe.text().to_string();
        let action = ac.active_text().unwrap_or_else(|| "accept".into());
        nft::add_rule(&proto, &src, &dst, &port, &action);
        dialog.close();
    });

    dialog.show();
}
