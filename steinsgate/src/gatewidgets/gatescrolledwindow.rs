use super::GateWidget;
use gtk4::builders::ScrolledWindowBuilder;
use gtk4::Align;
use gtk4::PolicyType;
use gtk4::ScrolledWindow;
pub struct GateScrolledWindow {
    pub vexpand: bool,
    pub min_content_width: i32,
    pub halign: Align,
    pub valign: Align,
    pub hscorllbar_policy: PolicyType,
    pub vscorllbar_policy: PolicyType,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}
impl Default for GateScrolledWindow {
    fn default() -> Self {
        GateScrolledWindow {
            vexpand: true,
            min_content_width: 360,
            halign: Align::Fill,
            valign: Align::Fill,
            hscorllbar_policy: PolicyType::Never,
            vscorllbar_policy: PolicyType::Always,
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}
impl GateWidget<ScrolledWindowBuilder> for GateScrolledWindow {
    fn prebuild(&self) -> ScrolledWindowBuilder {
        ScrolledWindow::builder()
            .vexpand(true)
            .min_content_width(360)
            .halign(self.halign)
            .valign(self.valign)
            .vscrollbar_policy(self.vscorllbar_policy)
            .hscrollbar_policy(self.hscorllbar_policy)
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .margin_top(self.margin_top)
            .margin_bottom(self.margin_bottom)
    }
}
