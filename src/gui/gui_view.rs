use iced::{
    Alignment::Center,
    Color, Element, Theme,
    widget::{button, center, column, row, text_input},
};

use iced::time::Duration;

use crate::gui::gui_logic::Gui;
use crate::gui::gui_logic::GuiLogic;
use crate::gui::gui_logic::Message;

use iced::Subscription;

pub fn run() {
    let settings = iced::Settings {
        ..Default::default()
    };

    let app = iced::application("BladeWay AutoClicker", Gui::update, Gui::view)
        .subscription(Gui::subscription)
        .settings(settings)
        .theme(|_| Theme::Dracula)
        .window_size(iced::Size::new(600., 300.));

    let _ = app.run();
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

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }
}
