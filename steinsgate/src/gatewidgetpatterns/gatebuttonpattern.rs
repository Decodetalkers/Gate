use super::GateWidgetPattern;
use super::HasLabel;
use gtk4::builders::ButtonBuilder;
use gtk4::prelude::*;
use gtk4::Button;
pub struct GateButtonPattern<'a> {
    pub text: &'a str,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}
impl<'a> Default for GateButtonPattern<'a> {
    fn default() -> Self {
        GateButtonPattern {
            text: "Hackthegate",
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}
impl<'a> GateWidgetPattern<ButtonBuilder> for GateButtonPattern<'a> {
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
