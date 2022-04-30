mod gateboxpattern;
mod gatebuttonpattern;
mod gatelabelpattern;
mod gatepopwindowpattern;
mod gatescrolledwindowpattern;
pub use gateboxpattern::GateBoxPattern;
pub use gatebuttonpattern::*;
pub use gatelabelpattern::*;
pub use gatepopwindowpattern::GatePopWindowPattern;
pub use gatescrolledwindowpattern::GateScrolledWindowPattern;
pub trait GateWidgetPattern<T> {
    fn prebuild(&self) -> T;
}
pub trait HasLabel {
    fn set_the_label(&self, input: &str, size: i32);
}
