use super::GateWidget;
use gtk4::Label;
pub struct GateLabel<'a> {
    pub fontsize: usize,
    pub text: &'a str,
    pub margin_start: i32,
    pub margin_end :i32,
    pub margin_top : i32,
    pub margin_bottom:i32,
}
impl<'a> Default for GateLabel<'a> {
    fn default() -> Self {
        GateLabel {
            fontsize: 10,
            text: "Hack to the gate",
            margin_start:0,
            margin_end:0,
            margin_top:0,
            margin_bottom:0,
        }
    }
}
impl<'a> GateWidget<Label> for GateLabel<'a> {
    fn build(&self) -> Label {
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
            .build()
    }
}
pub trait FontResize {
    fn set_font_label(&self, input :&str,size:i32);
}
impl FontResize for Label {
    fn set_font_label(&self, input: &str,size:i32) {
        if !self.uses_markup() {
            self.set_use_markup(true);
        };
        self.set_label(&format!("<span size='{}'>{}</span>",size,input));
    }
}