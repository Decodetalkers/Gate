//use gtk4::glib;
use gtk4::{prelude::*, Overlay};
//use std::borrow::Borrow;
//use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;
//use std::sync::Mutex;
use steinsgate::gatewidgets::*;

mod popuppage;
type Message = Rc<RefCell<(String, bool, i32)>>;
pub fn todo_page(overlay: Rc<Overlay>) -> Rc<gtk4::Box> {
    let containermax = GateBox {
        halign: gtk4::Align::Fill,
        valign: gtk4::Align::Fill,
        ..Default::default()
    }
    .prebuild()
    .build();
    let container = GateBox {
        halign: gtk4::Align::Fill,
        valign: gtk4::Align::Fill,
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

    let entry = gtk4::Entry::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .placeholder_text("Enter a Task")
        .secondary_icon_name("list-add-symbolic")
        .build();
    let states: Rc<RefCell<Vec<Message>>> = Rc::new(RefCell::new(Vec::new()));
    let stateclean = Rc::clone(&states);
    let containerrc = Rc::new(container);
    let containerrcclean = containerrc.clone();
    let overlayclean = overlay.clone();
    entry.connect_icon_press(move |entry, _icon| {
        let mut thestate = states.borrow_mut();
        let text = entry.text().to_string();
        let astate = Rc::new(RefCell::new((text, false, 0)));
        thestate.push(Rc::clone(&astate));
        containerrc.append(&*to_do_row(overlay.clone(), astate));
    });

    let cleanbutton = GateButton::default()
        .prebuild()
        .build()
        .set_onclick(move |_| {
            let mut states = stateclean.borrow_mut();
            let newstates: Vec<Message> = (*states)
                .iter()
                .filter(|messages| {
                    let message = messages.borrow();
                    let (_, isclicked, _) = *message;
                    !isclicked
                })
                .map(|input| input.clone())
                .collect();

            loop {
                match containerrcclean.last_child() {
                    Some(child) => containerrcclean.remove(&child),
                    None => break,
                }
            }
            for astate in &newstates {
                containerrcclean.append(&*to_do_row(overlayclean.clone(), astate.clone()));
            }
            *states = newstates;
        });
    containermax.append(&cleanbutton);
    containermax.append(&entry);
    containermax.append(&scrolled_window);
    Rc::new(containermax)
}
fn to_do_row(overlay: Rc<Overlay>, state: Message) -> Rc<gtk4::Box> {
    let input2 = state.borrow();
    let (input, _, time) = input2.clone();
    let thebox = GateBox {
        orientation: gtk4::Orientation::Horizontal,
        valign: gtk4::Align::Start,
        halign: gtk4::Align::Start,
        ..Default::default()
    }
    .prebuild()
    .build();
    let fontsize = 30111;
    //let check = RefCell::new(false);
    let labelprew = GateLabel {
        margin_end: 12,
        margin_top: 12,
        margin_start: 12,
        margin_bottom: 12,
        text: &input,
        fontsize,
    };
    drop(input2);
    let lable = labelprew.prebuild().build();
    let recordlabel = GateLabel {
        text: &time.to_string(),
        ..labelprew
    }
    .prebuild()
    .build();
    let updatelabel = Rc::new(recordlabel);
    let recordlabel = updatelabel.clone();
    let time2 = state.clone();
    let check_button = gtk4::CheckButton::builder().build();
    let popupbutton = GateButton {
        text: "Popup",
        margin_start: 15,
        margin_end: 15,
        margin_top: 15,
        margin_bottom: 15,
    }
    .prebuild()
    .build()
    .set_onclick(move |_| {
        overlay.add_overlay(&*popuppage::popup_page(
            time2.clone(),
            overlay.clone(),
            updatelabel.clone(),
            fontsize,
        ));
    });
    thebox.append(&check_button);
    thebox.append(&lable);
    thebox.append(&*recordlabel);
    thebox.append(&popupbutton);
    let label = Rc::new(lable);
    let recordlabelrc = Rc::new(recordlabel);
    let input = input.to_string();
    check_button.connect_toggled(move |_| {
        let mut change = state.borrow_mut();
        change.1 = !change.1;
        change.2 += 1;
        recordlabelrc.set_font_label(&change.2.to_string(), fontsize);
        let checked = change.1;
        //let mut checked = check.borrow_mut();
        if checked {
            label.set_font_label("done", fontsize);
        } else {
            label.set_font_label(&input, fontsize);
        }
    });
    Rc::new(thebox)
}
