use std::{
    io::{Error, Write, stdout},
    thread::spawn,
};

use colored::Colorize;
use crossterm::{
    Command,
    cursor::MoveTo,
    queue,
    style::{Print, Stylize},
    terminal::{self, Clear, ClearType, enable_raw_mode},
};

pub struct Screen {
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = terminal::size().unwrap();
        Self {
            width: w,
            height: h,
        }
    }

    pub fn init(&self) {
        //        enable_raw_mode().unwrap();
        self.clear_screen().unwrap();
        self.set_pos(0, 0).unwrap();
        self.welcome().unwrap();
    }

    pub fn clear_screen(&self) -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        self.set_pos(0, 0).unwrap();
        stdout().flush()?;

        Ok(())
    }

    pub fn print(&self, str: String) -> Result<(), Error> {
        Self::queue_command(Print(str))?;
        Ok(())
    }

    pub fn set_pos(&self, x: u16, y: u16) -> Result<(), Error> {
        Self::queue_command(MoveTo(x, y))?;
        Ok(())
    }

    fn flash() {}

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    fn welcome(&self) -> Result<(), Error> {
        self.print("mineswapper".to_string())?;
        self.print("\n\n\n\n".to_string())?;
        self.print("press enter to start game!\n".to_string())?;
        Ok(())
    }
}
