use super::GateWidget;
use gtk4::Align;
use gtk4::Box;
use gtk4::Orientation;
pub struct GateBox {
    pub orientation: Orientation,
    pub halign: Align,
    pub valign: Align,
    pub margin_start: i32,
    pub margin_end: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub spacing: i32,
}
impl Default for GateBox {
    fn default() -> Self {
        GateBox {
            orientation: Orientation::Vertical,
            halign: Align::Center,
            valign: Align::Center,
            margin_start: 0,
            margin_end: 0,
            margin_top: 0,
            margin_bottom: 0,
            spacing: 0,
        }
    }
}
impl GateWidget<Box> for GateBox {
    fn build(&self) -> Box {
        Box::builder()
            .orientation(self.orientation)
            .halign(self.halign)
            .valign(self.valign)
            .margin_top(self.margin_top)
            .margin_bottom(self.margin_bottom)
            .margin_start(self.margin_start)
            .margin_end(self.margin_end)
            .spacing(self.spacing)
            .build()
    }
}
