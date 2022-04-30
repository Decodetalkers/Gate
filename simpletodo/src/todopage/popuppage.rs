use std::rc::Rc;

use super::Message;
use gtk::prelude::*;
use gtk4 as gtk;
use steinsgate::gatewidgetpatterns::*;
use steinsgate::gatewidgets::*;
pub(super) fn popup_page<T>(
    input: Message,
    overlay: Rc<GatePopWindow>,
    updatelabel: Rc<T>,
    font: i32,
) -> Rc<gtk::Box>
where
    T: HasLabel + 'static,
{
    let number = input.borrow();
    let fontsize = 30111;
    let window = GateBoxPattern {
        margin_end: 15,
        margin_top: 15,
        margin_start: 15,
        margin_bottom: 15,
        ..Default::default()
    }
    .prebuild()
    .build();
    let label = GateLabelPattern {
        text: &number.2.to_string(),
        fontsize,
        ..Default::default()
    }
    .prebuild()
    .build();
    drop(number);
    window.append(&label);
    let rclabel = Rc::new(label);
    let button = GateButtonPattern {
        text: "Click it",
        margin_bottom: 15,
        margin_top: 15,
        margin_start: 15,
        margin_end: 15,
    }
    .prebuild()
    .build()
    .set_onclick(move |_| {
        let mut text = input.borrow_mut();
        text.2 += 1;
        rclabel.set_font_label(&text.2.to_string(), fontsize);
        updatelabel.set_the_label(&text.2.to_string(), font);
    });
    let window = Rc::new(window);
    let output = window.clone();
    let button2 = GateButtonPattern {
        text: "Return",
        margin_bottom: 15,
        margin_top: 15,
        margin_start: 15,
        margin_end: 15,
    }
    .prebuild()
    .build()
    .set_onclick(move |_| {
        overlay.set_page(0);
        overlay.pushback();
    });
    output.append(&button);
    output.append(&button2);

    output
}
