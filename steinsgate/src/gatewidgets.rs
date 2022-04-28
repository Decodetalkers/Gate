mod gatebox;
mod gatebutton;
mod gatelabel;
pub use gatebox::GateBox;
pub use gatebutton::GateButton;
pub use gatelabel::*;
pub trait GateWidget<T> {
    fn build(&self) -> T;
}
