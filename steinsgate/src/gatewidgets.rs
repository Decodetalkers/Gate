mod gatepopwindow;
use gtk4::{prelude::IsA, Notebook, Widget};
pub type GatePopWindow = Notebook;
pub trait PopUpWindow {
    fn popup<T>(&self, child: &T)
    where
        T: IsA<Widget>;
    fn pushback(&self);
}
