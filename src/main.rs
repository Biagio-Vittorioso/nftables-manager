mod nft;
mod ui;

use gtk4 as gtk;
use gtk::prelude::*;

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.example.nftables-manager")
        .build();

    app.connect_activate(|app| {
        ui::app_window::build_main_window(app);
    });

    app.run();
}
