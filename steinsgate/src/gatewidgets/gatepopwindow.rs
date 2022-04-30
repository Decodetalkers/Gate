use super::GatePopWindow;
use super::PopUpWindow;
use crate::gatewidgetpatterns::GateLabelPattern;
use crate::gatewidgetpatterns::GateWidgetPattern;
use gtk4::prelude::*;
impl PopUpWindow for GatePopWindow {
    fn popup<T>(&self, child: &T)
    where
        T: IsA<gtk4::Widget>,
    {
        self.append_page(child, Some(&GateLabelPattern::default().prebuild().build()));
    }
    fn pushback(&self) {
        self.remove_page(Some(1));
    }
}
