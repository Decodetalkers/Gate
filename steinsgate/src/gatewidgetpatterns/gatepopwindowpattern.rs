use super::GateWidgetPattern;
use gtk4::{builders::NotebookBuilder, *};
pub struct GatePopWindowPattern {
    pub halign: Align,
    pub valign: Align,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}
impl Default for GatePopWindowPattern {
    fn default() -> Self {
        GatePopWindowPattern {
            halign: Align::Fill,
            valign: Align::Fill,
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}
impl GateWidgetPattern<NotebookBuilder> for GatePopWindowPattern {
    fn prebuild(&self) -> NotebookBuilder {
        gtk4::Notebook::builder()
            .valign(self.valign)
            .halign(self.halign)
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_end(self.margin_end)
            .show_tabs(false)
    }
}
