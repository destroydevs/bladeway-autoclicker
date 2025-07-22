use std::{sync::mpsc, thread::JoinHandle};

use iced::{
    Alignment::Center,
    Color, Element, Theme,
    futures::lock::Mutex,
    widget::{button, center, column, row, text_input},
};
use iced::Subscription;

use std::sync::Arc;
use std::thread;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::key;

pub fn run() {
    let settings = iced::Settings {
        ..Default::default()
    };

    let app = iced::application("BladeWay AutoClicker", Gui::update, Gui::view)
        .settings(settings)
        .theme(|_| Theme::Dracula)
        .window_size(iced::Size::new(600., 300.));

    let _ = app.run();
}

struct Gui {
    input: String,
    is_running: bool,
    name: String,
    error: Option<String>,
    mpsc: (Sender<GuiUpdate>, Receiver<GuiUpdate>),
    clicker_state: bool,
    thread: Option<JoinHandle<()>>,
}

struct AppState {
    enabled: String,
    disabled: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            enabled: "Остановить".to_string(),
            disabled: "Включить".to_string(),
        }
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            input: "R".to_string(),
            is_running: false,
            name: AppState::default().disabled,
            error: None,
            mpsc: mpsc::channel::<GuiUpdate>(),
            clicker_state: false,
            thread: None,
        }
    }
}

#[derive(Debug, Clone)]
enum GuiUpdate {
    ErrorOccurred(String),
    ClickerStateChange,
    ClearError,
}

#[derive(Debug, Clone)]
enum Message {
    Run,
    Apply(String),
    Input(String),
}

impl Gui {
    fn view(&self) -> Element<Message> {
        let error = if let Some(e) = &self.error {
            e.clone()
        } else {
            String::from("")
        };

        let title = iced::widget::text("BladeWay AutoClicker").size(25);

        let label = iced::widget::text("На какую кнопку будет включаться кликер?");

        let input = text_input("R", &self.input)
            .on_input(Message::Input)
            .on_submit(Message::Apply(self.input.clone()))
            .width(25)
            .size(15);

        let run_btn = button(&*self.name).on_press(Message::Run);

        let error_text = iced::widget::text(error).color(Color::from_rgb8(240, 84, 30));

        let copyright = iced::widget::text("Copyright © 2025 BladeWay")
            .color(Color::from_rgb8(127, 133, 119))
            .size(10);

        let row = row![input, run_btn].spacing(3);

        let centered_column = column![title, label, row, error_text,]
            .align_x(Center)
            .spacing(10);

        iced::widget::column![center(centered_column), copyright,]
            .padding(10)
            .into()
    }
}
