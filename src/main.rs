mod clicker;
mod gui;
mod sound;
mod windows;
use crate::gui::gui_view;

fn main() {
    env_logger::init();
    gui_view::run();
}
