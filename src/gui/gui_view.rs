/*
    BladeWay Autoclicker Program
    Copyright (C) 2025  Evgeny K.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use iced::{
    Alignment::Center,
    Color, Element, Theme,
    widget::{button, center, column, row, text_input},
};

use crate::gui::gui_logic::Gui;
use crate::gui::gui_logic::GuiLogic;
use crate::gui::gui_logic::Message;

use iced::Subscription;

static ICON: &[u8] = include_bytes!("../.././icon.ico");

pub fn run() {
    let image = image::load_from_memory(ICON).unwrap();
    let (width, height) = (image.width(), image.height());
    let icon = iced::window::icon::from_rgba(image.to_rgba8().into_raw(), width, height).unwrap();

    let window_settings = iced::window::Settings {
        icon: Some(icon),
        ..Default::default()
    };

    let settings = iced::Settings {
        ..Default::default()
    };

    let app = iced::application("BladeWay AutoClicker", Gui::update, Gui::view)
        .subscription(Gui::subscription)
        .settings(settings)
        .theme(|_| Theme::Dracula)
        .window(window_settings)
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
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
    }
}
