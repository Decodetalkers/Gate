mod gatebox;
mod gatebutton;
mod gatelabel;
mod gatescrolledwindow;
pub use gatebox::GateBox;
pub use gatebutton::GateButton;
pub use gatelabel::*;
pub use gatescrolledwindow::GateScrolledWindow;
pub trait GateWidget<T> {
    fn prebuild(&self) -> T;
}
