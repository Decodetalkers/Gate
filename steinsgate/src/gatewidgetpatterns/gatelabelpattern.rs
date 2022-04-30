use super::GateWidgetPattern;
use super::HasLabel;
use gtk4::{builders::LabelBuilder, Label};
pub struct GateLabelPattern<'a> {
    pub fontsize: i32,
    pub text: &'a str,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}
impl<'a> Default for GateLabelPattern<'a> {
    fn default() -> Self {
        GateLabelPattern {
            fontsize: 10000,
            text: "Hack to the gate",
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}
impl<'a> GateWidgetPattern<LabelBuilder> for GateLabelPattern<'a> {
    fn prebuild(&self) -> LabelBuilder {
        Label::builder()
            .use_markup(true)
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_bottom(self.margin_bottom)
            .label(&format!(
                "<span size='{}'>{}</span>",
                self.fontsize, self.text
            ))
    }
}
pub trait FontResize {
    fn set_font_label(&self, input: &str, size: i32);
}
impl FontResize for Label {
    fn set_font_label(&self, input: &str, size: i32) {
        if !self.uses_markup() {
            self.set_use_markup(true);
        };
        self.set_label(&format!("<span size='{}'>{}</span>", size, input));
    }
}
impl HasLabel for Label {
    fn set_the_label(&self, input: &str, size: i32) {
        self.set_font_label(input, size);
    }
}
