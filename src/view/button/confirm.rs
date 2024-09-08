use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box, TextView, Orientation, Spinner};


use crate::view::loader::spinner::create_spinner_loader;

pub fn create_confirm_button(app: &Application) -> Button {
    let button = Button::new();
    button.set_label("Mostrar Loader");

    button.connect_clicked(move |_| {
      
    });

    button
}