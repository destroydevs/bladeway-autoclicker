use crate::clicker;
use crate::clicker::Clicker;
use crate::sound::sound_util;
use crate::windows::key;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub trait GuiLogic {
    fn update(&mut self, message: Message);
    fn check_callback(&mut self);
    fn show_error(&mut self, error: String);
    fn disable_clicker(&mut self);
    fn enable_clicker(&mut self);
}

pub struct Gui {
    pub input: String,
    pub is_running: bool,
    pub name: String,
    pub error: Option<String>,
    pub mpsc: (Sender<GuiUpdate>, Receiver<GuiUpdate>),
    pub click_counter: AtomicUsize,
    pub thread: Option<JoinHandle<()>>,
    pub clicker: Clicker,
    pub clicker_enabled: bool,
}

pub struct AppState {
    pub enabled: String,
    pub disabled: String,
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
        let point = clicker::get_cursor_position();
        Self {
            input: "R".to_string(),
            is_running: false,
            name: AppState::default().disabled,
            error: None,
            mpsc: mpsc::channel::<GuiUpdate>(),
            click_counter: AtomicUsize::new(0),
            thread: None,
            clicker: Clicker::new(point, 250),
            clicker_enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Run,
    Apply(String),
    Input(String),
    Tick,
}

#[derive(Debug, Clone)]
pub enum GuiUpdate {
    ErrorOccurred(String),
    ClickerStateChange,
    ClearError,
    ClickerActivation,
}

impl GuiLogic for Gui {
    fn update(&mut self, message: Message) {
        match message {
            Message::Run => {
                if !self.is_running {
                    self.enable_clicker();
                } else {
                    self.disable_clicker();
                }
            }
            Message::Apply(s) => {
                self.input = s;

                let key = self.input.get(0..1).unwrap_or("").to_string();

                if self.input.len() > 1 {
                    if let Some(_) = key::HookKey::from_str(&key) {
                        self.input = key;
                    } else {
                        self.show_error(format!("Невозможно распознать клавишу - {}", key));
                    }
                }
            }
            Message::Input(s) => {
                if s.len() > 1 {
                    return;
                }

                if !self.is_running {
                    self.input = s.to_uppercase();
                }
            }
            Message::Tick => {
                self.check_callback();
            }
        }

        self.check_callback();
    }

    fn disable_clicker(&mut self) {
        self.name = AppState::default().disabled;
        self.is_running = false;

        self.clicker.stop();

        self.click_counter
            .store(0, std::sync::atomic::Ordering::SeqCst);

        self.clicker_enabled = false;

        key::HookKey::unregister();
        if let Some(handle) = self.thread.take() {
            if handle.is_finished() {
                let _ = handle.join();
            } else {
                drop(handle);
            }
        }

        self.mpsc = mpsc::channel::<GuiUpdate>();

        sound_util::play_orb_sound();
    }

    fn enable_clicker(&mut self) {
        let inp = Arc::new(self.input.clone());
        let sender = self.mpsc.0.clone();
        let error_sender = sender.clone();

        if let Err(e) = key::register_key(&inp, move || {
            let _ = sender.send(GuiUpdate::ClickerActivation);
            let _ = sender.send(GuiUpdate::ClickerStateChange);
        }) {
            let _ = error_sender.send(GuiUpdate::ErrorOccurred(e.to_string()));
        };

        self.name = AppState::default().enabled;
        self.is_running = true;

        sound_util::play_orb_sound();
    }

    fn show_error(&mut self, error: String) {
        self.error = Some(error);

        let sender = self.mpsc.0.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let _ = sender.send(GuiUpdate::ClearError);
        });
    }

    fn check_callback(&mut self) {
        while let Ok(update) = self.mpsc.1.try_recv() {
            match update {
                GuiUpdate::ErrorOccurred(err) => {
                    self.show_error(err);
                    self.is_running = true; // will be changed to false in end
                    self.name = AppState::default().disabled;

                    self.input = "R".to_string();

                    if let Some(handle) = self.thread.take() {
                        if handle.is_finished() {
                            if let Err(e) = handle.join() {
                                self.show_error(format!("Упс... Ошибка: {:?}", e));
                            }
                        }
                    }

                    key::HookKey::unregister();
                }
                GuiUpdate::ClickerActivation => {
                    if !self.clicker_enabled {
                        self.clicker
                            .set_other_position(clicker::get_cursor_position());

                        self.clicker.run();

                        self.clicker_enabled = true;
                    } else {
                        self.clicker.stop();

                        self.clicker_enabled = false;
                    }
                }
                GuiUpdate::ClickerStateChange => {
                    if self.click_counter.load(std::sync::atomic::Ordering::SeqCst) >= 1 {
                        self.disable_clicker();
                    } else {
                        self.click_counter
                            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                }
                GuiUpdate::ClearError => {
                    self.error = None;
                }
            }
        }
    }
}
