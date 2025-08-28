use std::process::Command;
use std::io::{self, Write};

pub fn clear_terminal() {
    if cfg!(windows) {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else if cfg!(unix) {
        let _ = Command::new("clear").status();
    }
    io::stdout().flush().unwrap();
}