use gtk4::prelude::*;
use gtk4::Application;

use crate::ui::window::build_main_window;

pub fn run() {

    let app = Application::builder()
        .application_id("com.nftables.manager")
        .build();

    app.connect_activate(|app| {
        let window = build_main_window(app);
        window.present();
    });

    app.run();
}
