use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
//use gtk::subclass::*;
mod todopage;
use steinsgate::gatewidgets::{GateBox, GateScrolledWindow, GateWidget};
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
    let containermax = GateBox {
        halign: gtk::Align::Fill,
        valign: gtk::Align::Fill,
        ..Default::default()
    }
    .prebuild()
    .build();
    let container = GateBox {
        halign: gtk::Align::Fill,
        valign: gtk::Align::Fill,
        margin_end: 15,
        margin_top: 15,
        margin_start: 15,
        margin_bottom: 15,
        ..Default::default()
    }
    .prebuild()
    .build();
    let scrolled_window = GateScrolledWindow::default()
        .prebuild()
        .child(&container)
        .build();

    let entry = gtk::Entry::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .placeholder_text("Enter a Task")
        .secondary_icon_name("list-add-symbolic")
        .build();
    let containerrc = std::rc::Rc::new(container);
    entry.connect_icon_press(move |entry, _icon| {
        containerrc.append(&*todopage::to_do_row(&entry.text().to_string()));
    });
    containermax.append(&entry);
    containermax.append(&scrolled_window);
    //container.append(&*mybutton::rc_labey());
    //container.append(&*mybutton::rc_button());
    //container.append(&*mybutton::rc_button2());
    //container.append(&*mybutton::to_do_row("Hack"));
    window.set_child(Some(&containermax));
    // Show the window.
    window.show();
}
