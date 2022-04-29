mod gatebox;
mod gatebutton;
mod gatelabel;
mod gatescrolledwindow;
pub use gatebox::GateBox;
pub use gatebutton::*;
pub use gatelabel::*;
pub use gatescrolledwindow::GateScrolledWindow;
pub trait GateWidget<T> {
    fn prebuild(&self) -> T;
}
pub trait HasLabel {
    fn set_the_label(&self, input: &str, size: i32);
}
