use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation};


mod util;
mod view;

use crate::util::constant::APP_TITLE;

use crate::view::button::confirm::create_confirm_button;

use crate::view::text_area::songLyrics::create_text_area;


fn main() {
    let app = Application::new(
        Some("com.example.buttons"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let main_window = ApplicationWindow::new(app);
        main_window.set_title(Some(APP_TITLE));
        main_window.set_default_size(300, 200);

        let vbox = Box::new(Orientation::Vertical, 5);

        let text_area = create_text_area(app);
        let button = create_confirm_button(app);

        vbox.append(&text_area);
        vbox.append(&button);

        main_window.set_child(Some(&vbox));
        main_window.show();
    });
    
    app.run();
}
