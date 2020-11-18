use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CEvent, KeyEvent};

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap crossterm input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    // poll for tick rate duration, if no events, sent tick event.
                    let timeout = config
                        .tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));
                    if event::poll(timeout).unwrap() {
                        if let CEvent::Key(key) = event::read().unwrap() {
                            if let Err(err) = tx.send(Event::Input(key)) {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                    }
                    if last_tick.elapsed() >= config.tick_rate {
                        tx.send(Event::Tick).unwrap();
                        last_tick = Instant::now();
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
