pub mod event;
pub mod my_gauge;
pub mod renderer;
pub mod stateful_list;

use crossterm::event::{self as cevent, Event as CEvent, KeyCode, KeyEvent};
use failure::Error;

pub fn read_line() -> Result<String, Error> {
    let mut line = String::new();
    while let CEvent::Key(KeyEvent { code, .. }) = cevent::read()? {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    return Ok(line);
}
