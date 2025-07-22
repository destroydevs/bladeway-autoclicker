use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use mouse_rs::{Mouse, types::keys::Keys};

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

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from(x: i32, y: i32) -> Self {
        Point { x, y }
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

    pub fn run(&self) {
        if self.enabled.load(Ordering::SeqCst) {
            return;
        }

        self.enabled.store(true, Ordering::SeqCst);

        let mouse = Arc::clone(&self.mouse);
        let enabled = Arc::clone(&self.enabled);
        let position = self.position;
        let mouse_click = Arc::clone(&self.mouse);
        let speed_click = self.speed;
        thread::spawn( move ||{
             thread::sleep(Duration::from_millis(speed_click as u64));

            let _ = mouse_click.click(&Keys::LEFT);
        });

       thread::spawn(move || {
            while enabled.load(Ordering::SeqCst) {
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

