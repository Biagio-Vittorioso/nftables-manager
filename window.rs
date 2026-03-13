use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation};

use super::sidebar::build_sidebar;
use super::rules_table::build_rules_table;
pub fn build_main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("NFTables Manager")
        .default_width(1000)
        .default_height(600)
        .build();
    let layout = Box::new(Orientation::Horizontal, 0);
    let sidebar = build_sidebar();
    let rules = build_rules_table();
    layout.append(&sidebar);
    layout.append(&rules);
    window.set_child(Some(&layout));
    window
}
