use super::GateWidget;
use super::HasLabel;
use gtk4::builders::ButtonBuilder;
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
impl<'a> GateWidget<ButtonBuilder> for GateButton<'a> {
    fn prebuild(&self) -> ButtonBuilder {
        Button::builder()
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_end(self.margin_end)
            .label(self.text)
    }
}
pub trait OnClick {
    fn set_onclick<T>(self, onclick: T) -> Self
    where
        T: Fn(&Button) + 'static;
}
impl<'a> OnClick for Button {
    fn set_onclick<T>(self, onclick: T) -> Self
    where
        T: Fn(&Button) + 'static,
    {
        self.connect_clicked(onclick);
        self
    }
}
impl HasLabel for Button {
    fn set_the_label(&self, input: &str, _size: i32) {
        self.set_label(input);
    }
}
