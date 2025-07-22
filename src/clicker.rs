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

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use mouse_rs::{
    Mouse,
    types::{Point, keys::Keys},
};

use crate::windows::mouse_util;

pub struct Clicker {
    position: (i32, i32),
    speed: u32,
    mouse: Arc<Mouse>,
    enabled: Arc<AtomicBool>,
}

impl Default for Clicker {
    fn default() -> Self {
        Clicker {
            position: (0, 0),
            speed: 250,
            mouse: Arc::new(mouse_rs::Mouse::new()),
            enabled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Clicker {
    pub fn new(point: Point, speed: u32) -> Self {
        Clicker {
            position: (point.x, point.y),
            speed,
            ..Default::default()
        }
    }

    pub fn set_other_position(&mut self, point: Point) {
        self.position = (point.x, point.y);
    }

    pub fn run(&self) {
        if self.enabled.load(Ordering::SeqCst) {
            return;
        }

        self.enabled.store(true, Ordering::SeqCst);

        let click_enabled = Arc::clone(&self.enabled);
        let mouse_click = Arc::clone(&self.mouse);
        let speed_click = self.speed;
        thread::spawn(move || {
            while click_enabled.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(speed_click as u64));

                let _ = mouse_click.click(&Keys::LEFT);
            }
        });

        self.disable_movement();
    }

    pub fn stop(&self) {
        if !self.enabled.load(Ordering::SeqCst) {
            return;
        }

        self.enable_movement();

        self.enabled.store(false, Ordering::SeqCst);
    }

    pub fn disable_movement(&self) {
        mouse_util::block_mouse();
    }

    pub fn enable_movement(&self) {
        mouse_util::unblock_mouse();
    }
}

pub fn get_cursor_position() -> Point {
    let mouse = Mouse::new();
    mouse.get_position().unwrap_or(Point { x: 0, y: 0 })
}
