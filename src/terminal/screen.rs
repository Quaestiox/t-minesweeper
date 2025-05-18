use std::{
    io::{Error, Write, stdout},
    thread::spawn,
    time::Duration,
};

use colored::Colorize;
use crossterm::{
    Command,
    cursor::MoveTo,
    queue,
    style::{Print, Stylize},
    terminal::{self, Clear, ClearType, enable_raw_mode},
};

use crate::game::{cfg::Config, game::Game};

use super::input::input;

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
        stdout().flush().unwrap();
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
        self.banner()?;
        self.print("\n\n\n\n".to_string())?;
        Ok(())
    }

    fn banner(&self) -> Result<(), Error> {
        self.print(
            "████████╗   ███╗   ███╗██╗███╗   ██╗███████╗███████╗██╗    ██╗███████╗███████╗██████╗ ███████╗██████╗ \n".to_string()
        )?;
        self.print(
            "╚══██╔══╝   ████╗ ████║██║████╗  ██║██╔════╝██╔════╝██║    ██║██╔════╝██╔════╝██╔══██╗██╔════╝██╔══██╗\n".to_string()
        )?;
        self.print(
            "   ██║█████╗██╔████╔██║██║██╔██╗ ██║█████╗  ███████╗██║ █╗ ██║█████╗  █████╗  ██████╔╝█████╗  ██████╔╝\n".to_string()
        )?;
        self.print(
            "   ██║╚════╝██║╚██╔╝██║██║██║╚██╗██║██╔══╝  ╚════██║██║███╗██║██╔══╝  ██╔══╝  ██╔═══╝ ██╔══╝  ██╔══██╗\n".to_string()
        )?;
        self.print(
            "   ██║      ██║ ╚═╝ ██║██║██║ ╚████║███████╗███████║╚███╔███╔╝███████╗███████╗██║     ███████╗██║  ██║\n".to_string()
        )?;
        self.print(
            "   ╚═╝      ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚══╝╚══╝ ╚══════╝╚══════╝╚═╝     ╚══════╝╚═╝  ╚═╝\n".to_string()
        )?;
        self.print(
            "                                                                                                      \n".to_string()
        )?;
        stdout().flush();
        Ok(())
    }

    pub fn success(&self, dura: Duration) -> Result<(), Error> {
        self.clear_screen()?;
        self.set_pos(0, 0)?;
        self.print("Success\n".dark_green().to_string())?;
        let p = format!("use time:{:?}", dura);
        self.print(p.dark_cyan().to_string())?;
        self.print("\n\n\n\n".to_string())?;
        stdout().flush()?;
        self.after()?;
        Ok(())
    }

    pub fn after(&self) -> Result<(), Error> {
        self.print("press enter to continue\n".to_string())?;
        self.print("press q to quit".to_string())?;

        stdout().flush()?;
        Ok(())
    }

    pub fn die(&self) -> Result<(), Error> {
        self.clear_screen()?;
        self.set_pos(0, 0)?;
        self.print("You Die !!!!!".dark_red().to_string())?;
        self.print("\n\n\n\n".to_string())?;
        stdout().flush()?;
        self.after()?;
        Ok(())
    }

    pub fn choose(&self) -> Config {
        self.print("press e to select EASY mode\n".dark_green().to_string());
        self.print("press n to select NORMAL mode\n".dark_cyan().to_string());
        self.print("press h to select HARD mode\n".dark_red().to_string());
        self.print("\n\n\n\n".to_string());
        stdout().flush();
        let mut cfg: Config = Config::easy();
        let mut ch;
        loop {
            ch = input();
            if ch == "e".to_string() {
                cfg = Config::easy();
                break;
            } else if ch == "n".to_string() {
                cfg = Config::normal();
                break;
            } else if ch == "h".to_string() {
                cfg = Config::hard();
                break;
            }
        }
        cfg
    }
}
