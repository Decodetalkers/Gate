//use gtk4::glib;
use gtk4::prelude::*;
//use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;
//use std::sync::Mutex;
use steinsgate::gatewidgets::*;
//struct Student {
//    name: String,
//    age: i32,
//}
//impl Student {
//    fn grow(&mut self) {
//        self.age += 1;
//    }
//    fn messages(&self) -> String {
//        format!("{} is {} year old", self.name, self.age)
//    }
//}
//static THE_STUDENT: Lazy<Mutex<Student>> = Lazy::new(|| {
//    Mutex::new(Student {
//        name: "Mike".to_string(),
//        age: 0,
//    })
//});
pub fn to_do_row(input: &str) -> Rc<gtk4::Box> {
    let thebox = GateBox {
        orientation: gtk4::Orientation::Horizontal,
        valign: gtk4::Align::Start,
        halign: gtk4::Align::Start,
        ..Default::default()
    }
    .prebuild()
    .build();
    let fontsize = 30111;
    let check = RefCell::new(false);
    let time = RefCell::new(0);
    let labelprew = GateLabel {
        margin_end: 12,
        margin_top: 12,
        margin_start: 12,
        margin_bottom: 12,
        text: input,
        fontsize,
    };
    let lable = labelprew.prebuild().build();
    let recordlabel = GateLabel {
        text: "0",
        ..labelprew
    }
    .prebuild()
    .build();
    let check_button = gtk4::CheckButton::builder().build();
    thebox.append(&check_button);
    thebox.append(&lable);
    thebox.append(&recordlabel);
    let label = Rc::new(lable);
    let recordlabelrc = Rc::new(recordlabel);
    let input = input.to_string();
    check_button.connect_toggled(move |_| {
        let mut time = time.borrow_mut();
        *time += 1;
        recordlabelrc.set_font_label(&time.to_string(), fontsize);
        let mut checked = check.borrow_mut();
        *checked = !*checked;
        if *checked {
            label.set_font_label("done", fontsize);
        } else {
            label.set_font_label(&input, fontsize);
        }
    });
    Rc::new(thebox)
}
//pub fn to_do_top() -> Rc<gtk4::Box> {
//    let thebox = GateBox {
//        spacing: 6,
//        orientation: gtk4::Orientation::Vertical,
//        margin_top: 12,
//        margin_bottom: 12,
//        margin_start: 12,
//        margin_end: 12,
//        ..Default::default()
//    }
//    .build();
//
//    Rc::new(GateBox::default().build())
//}
//pub fn rc_button() -> Rc<gtk4::Button> {
//    let student = RefCell::new(Student {
//        name: "cht".to_string(),
//        age: 0,
//    });
//    let message = student.borrow().messages();
//    let button = GateButton {
//        text: &message,
//        ..Default::default()
//    }
//    .define(move |button| {
//        let mut temp = student.borrow_mut();
//        if temp.age > 100 {
//            button.set_sensitive(false);
//            return;
//        }
//        //let mut student = THE_STUDENT.lock().unwrap();
//        temp.grow();
//        button.set_label(&temp.messages());
//    });
//
//    Rc::new(button)
//}
//pub fn rc_button2() -> Rc<gtk4::Button> {
//    let temp = THE_STUDENT.lock().unwrap();
//    let button = GateButton {
//        text: &temp.messages(),
//        margin_top: 20,
//        ..Default::default()
//    }
//    .define(move |button| {
//        let mut temp = THE_STUDENT.lock().unwrap();
//        //let mut student = THE_STUDENT.lock().unwrap();
//        if temp.age > 100 {
//            button.set_sensitive(false);
//            return;
//        }
//        temp.grow();
//        button.set_label(&temp.messages());
//        drop(temp)
//    });
//    //let button = gtk4::Button::builder().label(&temp.messages()).build();
//    drop(temp);
//    Rc::new(button)
//}
//pub fn rc_labey() -> Rc<gtk4::Label> {
//    use std::time::Duration;
//    let temp = THE_STUDENT.lock().unwrap();
//    let label = GateLabel {
//        text: &temp.messages(),
//        fontsize: 11803,
//        ..Default::default()
//    }
//    .build();
//    //let label = gtk4::Label::builder().label(&temp.messages()).build();
//    drop(temp);
//    let sample = Rc::new(label);
//    let output = Rc::clone(&sample);
//    let tick = move || {
//        let temp = THE_STUDENT.lock().unwrap();
//        if temp.age > 100 {
//            sample.set_font_label("You dead", 101803);
//            return glib::Continue(false);
//        }
//        sample.set_font_label(&temp.messages(), 11803);
//        drop(temp);
//        glib::Continue(true)
//    };
//    glib::timeout_add_local(Duration::from_millis(1), tick);
//    output
//}
