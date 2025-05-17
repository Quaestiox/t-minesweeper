use std::{
    io::{self, Read},
    process::exit,
};

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn input() -> String {
    let mut buffer = [0; 1024];
    io::stdin().read(&mut buffer).ok();
    let f = true;
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
                        KeyCode::Char(c) => match c {
                            'q' if modifiers == KeyModifiers::CONTROL => {
                                res = "quit".to_string();
                                break;
                            }
                            //                                '1'..='9' => return c,
                            _ => (),
                        },
                        KeyCode::Enter => {
                            println!("Enter detected");
                            res = "enter".to_string();
                            break;
                        }

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
