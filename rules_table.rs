use gtk4::prelude::*;
use gtk4::{Box, Orientation, ListStore, TreeView, TreeViewColumn, CellRendererText, Button, ScrolledWindow};

use crate::nft;
use serde_json::Value;

pub fn build_rules_table() -> Box {

    let container = Box::new(Orientation::Vertical, 5);

    let reload_btn = Button::with_label("Reload Rules");

    let store = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_vexpand(true);

    let columns = ["Source","Destination","Protocol","Port","Action"];
    for (i, title) in columns.iter().enumerate() {
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.set_title(title);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", i as i32);
        tree.append_column(&column);
    }

    let scroll = ScrolledWindow::builder()
        .child(&tree)
        .vexpand(true)
        .build();

    container.append(&reload_btn);
    container.append(&scroll);

    reload_btn.connect_clicked(move |_| {
        store.clear();
        if let Ok(json) = nft::get_ruleset() {
            if let Some(arr) = json.get("nftables").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(rule) = item.get("rule") {
                        let src = rule.get("src").and_then(|v| v.as_str()).unwrap_or("any").to_string();
                        let dst = rule.get("dst").and_then(|v| v.as_str()).unwrap_or("any").to_string();
                        let proto = rule.get("protocol").and_then(|v| v.as_str()).unwrap_or("any").to_string();
                        let port = rule.get("dport").map(|v| v.to_string()).unwrap_or("any".to_string());
                        let action = rule.get("verdict").and_then(|v| v.as_str()).unwrap_or("any").to_string();
                        store.insert_with_values(None,
                            &[0,1,2,3,4],
                            &[&src,&dst,&proto,&port,&action]);
                    }
                }
            }
        }
    });

    container
}
