//use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
//use gtk::subclass::*;
mod todopage;
fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hello, World!")
        .build();
    let overlay = gtk::Overlay::builder().build();
    window.set_child(Some(&overlay));
    let overlayrc = Rc::new(overlay);
    let overlayrc1 = overlayrc.clone();
    //let overlayrc2 = overlayrc.clone();
    overlayrc.add_overlay(&*todopage::todo_page(overlayrc1));

    // Show the window.
    window.show();
}
