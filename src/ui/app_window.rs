use crate::nft::{load_filter_rows, load_nat_rows, restart_nftables_service, DisplayRow};
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::rc::Rc;

pub fn build_main_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Firewall Configuration")
        .default_width(1280)
        .default_height(860)
        .build();

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let top_bar = build_top_bar();
    let switcher = gtk::StackSwitcher::new();
    let stack = gtk::Stack::new();

    switcher.set_stack(Some(&stack));
    switcher.set_halign(gtk::Align::Start);
    switcher.set_margin_start(28);
    switcher.set_margin_top(16);
    switcher.set_margin_bottom(0);

    let nat_page = build_rules_page(
        ["Type", "Protocol", "Source", "Destination", "Translation"],
        "IP Nat",
    );
    let filter_page = build_rules_page(
        ["Type", "Protocol", "Source", "Destination", "Details"],
        "IP Net filter",
    );
    let ports_page = build_rules_page(
        ["Port", "Protocol", "Source", "Destination", "Action"],
        "Ports",
    );
    let icmp_page = build_rules_page(
        ["Type", "Protocol", "Source", "Destination", "Action"],
        "ICMP Types",
    );
    let direct_page = build_rules_page(
        ["Type", "Protocol", "Source", "Destination", "Action"],
        "Direct Configuration",
    );

    stack.add_titled(&nat_page.root, Some("nat"), "IP Nat");
    stack.add_titled(&filter_page.root, Some("filter"), "IP Net Filter");
    stack.add_titled(&ports_page.root, Some("ports"), "Ports");
    stack.add_titled(&icmp_page.root, Some("icmp"), "ICMP Types");
    stack.add_titled(&direct_page.root, Some("direct"), "Direct Configuration");

    let reload_btn_top = top_bar.reload_button.clone();
    let reload_btn_bottom = nat_page.reload_button.clone();
    let apply_btn = nat_page.apply_button.clone();
    let close_btn = nat_page.close_button.clone();

    let nat_store = nat_page.store.clone();
    let filter_store = filter_page.store.clone();
    let ports_store = ports_page.store.clone();
    let icmp_store = icmp_page.store.clone();
    let direct_store = direct_page.store.clone();

    let refresh_all = Rc::new(move || {
        match load_nat_rows() {
            Ok(rows) => populate_store(&nat_store, &rows),
            Err(e) => populate_store(
                &nat_store,
                &[DisplayRow {
                    col1: "ERROR".into(),
                    col2: "-".into(),
                    col3: e,
                    col4: "-".into(),
                    col5: "-".into(),
                }],
            ),
        }

        match load_filter_rows() {
            Ok(rows) => populate_store(&filter_store, &rows),
            Err(e) => populate_store(
                &filter_store,
                &[DisplayRow {
                    col1: "ERROR".into(),
                    col2: "-".into(),
                    col3: e,
                    col4: "-".into(),
                    col5: "-".into(),
                }],
            ),
        }

        populate_store(&ports_store, &[]);
        populate_store(&icmp_store, &[]);
        populate_store(&direct_store, &[]);
    });

    {
        let refresh = refresh_all.clone();
        reload_btn_top.connect_clicked(move |_| {
            let _ = restart_nftables_service();
            refresh();
        });
    }

    {
        let refresh = refresh_all.clone();
        reload_btn_bottom.connect_clicked(move |_| {
            let _ = restart_nftables_service();
            refresh();
        });
    }

    {
        let window_weak = window.downgrade();
        close_btn.connect_clicked(move |_| {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
        });
    }

    apply_btn.connect_clicked(|_| {
        // Placeholder visivo: la GUI ha il pulsante, la logica si può aggiungere dopo.
    });

    refresh_all();

    root.append(&top_bar.root);
    root.append(&switcher);
    root.append(&stack);

    window.set_child(Some(&root));
    window.present();
}

struct TopBar {
    root: gtk::Box,
    reload_button: gtk::Button,
}

fn build_top_bar() -> TopBar {
    let root = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    root.set_margin_top(18);
    root.set_margin_start(28);
    root.set_margin_end(28);
    root.set_margin_bottom(12);

    let config_label = gtk::Label::new(Some("Configuration:"));
    config_label.set_xalign(0.0);

    let reload_button = gtk::Button::with_label("RELOAD");
    reload_button.set_size_request(160, 44);

    root.append(&config_label);
    root.append(&reload_button);

    TopBar { root, reload_button }
}

struct RulesPage {
    root: gtk::Box,
    store: gio::ListStore,
    reload_button: gtk::Button,
    apply_button: gtk::Button,
    close_button: gtk::Button,
}

fn build_rules_page(columns: [&str; 5], title: &str) -> RulesPage {
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 0);
    outer.set_margin_start(20);
    outer.set_margin_end(20);
    outer.set_margin_top(14);
    outer.set_margin_bottom(20);

    let frame = gtk::Frame::new(None);
    frame.set_hexpand(true);
    frame.set_vexpand(true);

    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    main_box.set_margin_top(18);
    main_box.set_margin_bottom(18);
    main_box.set_margin_start(18);
    main_box.set_margin_end(18);

    let store = gio::ListStore::new::<glib::BoxedAnyObject>();
    let selection = gtk::SingleSelection::new(Some(store.clone()));
    let view = gtk::ColumnView::new(Some(selection));

    view.set_hexpand(true);
    view.set_vexpand(true);

    view.append_column(&make_column(columns[0], |r| &r.col1));
    view.append_column(&make_column(columns[1], |r| &r.col2));
    view.append_column(&make_column(columns[2], |r| &r.col3));
    view.append_column(&make_column(columns[3], |r| &r.col4));
    view.append_column(&make_column(columns[4], |r| &r.col5));

    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(520);
    scrolled.set_child(Some(&view));

    let actions = gtk::Box::new(gtk::Orientation::Vertical, 10);
    actions.set_margin_start(16);
    actions.set_size_request(180, -1);

    let add_btn = action_button("Add");
    let edit_btn = action_button("Edit...");
    let delete_btn = action_button("Delete");
    let up_btn = action_button("Move Up");
    let down_btn = action_button("Move Down");

    actions.append(&add_btn);
    actions.append(&edit_btn);
    actions.append(&delete_btn);
    actions.append(&up_btn);
    actions.append(&down_btn);

    add_btn.set_sensitive(false);
    edit_btn.set_sensitive(false);
    delete_btn.set_sensitive(false);
    up_btn.set_sensitive(false);
    down_btn.set_sensitive(false);

    main_box.append(&scrolled);
    main_box.append(&actions);

    frame.set_child(Some(&main_box));

    let bottom_bar = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    bottom_bar.set_halign(gtk::Align::End);
    bottom_bar.set_margin_top(16);

    let reload_button = action_button("Reload");
    let apply_button = action_button("Apply");
    let close_button = action_button("Close");

    bottom_bar.append(&reload_button);
    bottom_bar.append(&apply_button);
    bottom_bar.append(&close_button);

    outer.append(&frame);
    outer.append(&bottom_bar);

    RulesPage {
        root: outer,
        store,
        reload_button,
        apply_button,
        close_button,
    }
}

fn action_button(label: &str) -> gtk::Button {
    let btn = gtk::Button::with_label(label);
    btn.set_size_request(140, 48);
    btn
}

fn make_column(
    title: &str,
    getter: fn(&DisplayRow) -> &str,
) -> gtk::ColumnViewColumn {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, item| {
        let label = gtk::Label::new(None);
        label.set_xalign(0.0);
        label.set_margin_start(12);
        label.set_margin_end(12);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        item.set_child(Some(&label));
    });

    factory.connect_bind(move |_, item| {
        let obj = item
            .item()
            .and_downcast::<glib::BoxedAnyObject>()
            .expect("BoxedAnyObject mancante");

        let row = obj.borrow::<DisplayRow>();

        let label = item
            .child()
            .and_downcast::<gtk::Label>()
            .expect("Label mancante");

        label.set_text(getter(&row));
    });

    gtk::ColumnViewColumn::new(Some(title), Some(factory))
}

fn populate_store(store: &gio::ListStore, rows: &[DisplayRow]) {
    store.remove_all();
    for row in rows {
        store.append(&glib::BoxedAnyObject::new(row.clone()));
    }
}
