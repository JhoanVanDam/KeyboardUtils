use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

mod util;

use crate::util::constant::APP_TITLE;

fn main() {
    let app = Application::builder()
        .application_id("com.example.gtk-rs")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_TITLE)
            .default_width(768)
            .default_height(500)
            .build();

        let button = Button::with_label("Click me!");
        window.set_child(Some(&button));

        window.show();
    });

    app.run();
}
