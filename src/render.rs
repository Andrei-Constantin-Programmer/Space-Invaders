use std::io::{Stdout, Write};

use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{ClearType, Clear}, cursor::MoveTo};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, column) in current_frame.iter().enumerate() {
        for (y, to_animate) in column.iter().enumerate() {
            if *to_animate != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *to_animate);
            }
        }
    }

    stdout.flush().unwrap();
}