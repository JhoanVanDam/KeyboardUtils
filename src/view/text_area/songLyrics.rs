use gtk4::prelude::*;
use gtk4::Application;
use gtk4::TextView;

pub fn create_text_area(app: &Application) -> TextView {
    let text_view = TextView::new();
    text_view.set_wrap_mode(gtk4::WrapMode::Word);
    text_view.set_vexpand(true);

 
    text_view
}