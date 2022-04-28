use super::GateWidget;
use gtk4::prelude::*;
use gtk4::Button;
pub struct GateButton<'a> {
    pub text: &'a str,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}
impl<'a> Default for GateButton<'a> {
    fn default() -> Self {
        GateButton {
            text: "Hackthegate",
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}
impl<'a> GateWidget<Button> for GateButton<'a> {
    fn build(&self) -> Button {
        Button::builder()
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_end(self.margin_end)
            .label(self.text)
            .build()
    }
}
impl<'a> GateButton<'a> {
    pub fn define<T>(&self, onclick: T) -> Button
    where
        T: Fn(&Button) -> () + 'static,
    {
        let button = Button::builder()
            .label(self.text)
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_end(self.margin_end)
            .build();
        button.connect_clicked(onclick);
        button
    }
}
