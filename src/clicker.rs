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

        let mouse = Arc::clone(&self.mouse);
        let click_enabled = Arc::clone(&self.enabled);
        let move_enabled = Arc::clone(&self.enabled);
        let position = self.position;
        let mouse_click = Arc::clone(&self.mouse);
        let speed_click = self.speed;
        thread::spawn(move || {
            while click_enabled.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(speed_click as u64));

                let _ = mouse_click.click(&Keys::LEFT);
            }
        });

        thread::spawn(move || {
            while move_enabled.load(Ordering::SeqCst) {
                let _ = mouse.move_to(position.0, position.1);
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn stop(&self) {
        if !self.enabled.load(Ordering::SeqCst) {
            return;
        }

        self.enabled.store(false, Ordering::SeqCst);
    }
}

pub fn get_cursor_position() -> Point {
    let mouse = Mouse::new();
    mouse.get_position().unwrap_or(Point { x: 0, y: 0 })
}
