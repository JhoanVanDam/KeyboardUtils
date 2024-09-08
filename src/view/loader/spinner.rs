use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box, TextView, Orientation, Spinner};


pub fn create_spinner_loader() -> Spinner {
    let spinner = Spinner::new();
    spinner.set_hexpand(true);
    spinner.set_vexpand(true);
    spinner.start(); // Inicia la animación del spinner

    spinner
}