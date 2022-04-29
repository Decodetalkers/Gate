use std::rc::Rc;

use gtk4 as gtk;
use gtk::Label;
use steinsgate::gatewidgets::*;
pub fn videopage() -> Rc<Label> {
    Rc::new(GateLabel::default().prebuild().build())
}
