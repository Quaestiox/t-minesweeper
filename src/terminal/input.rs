use std::{
    io::{self, Read},
    process::exit,
};

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn input() -> String {
    //    let mut buffer = [0; 1024];
    //   io::stdin().read(&mut buffer).ok();
    //    let f = true;
    enable_raw_mode().unwrap();
    let mut res = String::new();
    loop {
        if let Ok(event) = read() {
            match event {
                Event::Key(key_event) => match key_event {
                    KeyEvent {
                        code,
                        modifiers,
                        kind,
                        state,
                    } => match code {
                        KeyCode::Char(c) => {
                            res = c.to_string();
                            break;
                        }
                        KeyCode::Enter => (),
                        _ => (),
                    },
                },
                _ => (),
            }
        }
    }

    disable_raw_mode().unwrap();
    res
}
