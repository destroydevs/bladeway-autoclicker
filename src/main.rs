mod clicker;
mod gui;
mod sound;
mod windows;
use crate::gui::gui_view;
use crate::windows::key;

fn main() {
    env_logger::init();
    key::run_message_loop();
    gui_view::run();
}
