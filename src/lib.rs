use std::{error::Error, io::Stdout};

use crossterm::{
    style::{self, Colorize, Print, StyledContent},
    ExecutableCommand, QueueableCommand,
};

pub mod de;
pub mod kernel;
pub mod memory;
pub mod name;
pub mod os;
pub mod shell;
pub mod styled_data;
pub mod uptime;
pub mod wm;

pub fn print_styled_text(
    out: &mut Stdout,
    vec: &Vec<StyledContent<String>>,
    padding: usize,
) -> Result<(), Box<dyn Error>> {
    out.execute(Print(" ".repeat(padding)))?;
    for val in vec {
        out.queue(style::PrintStyledContent(val.clone()))?;
    }
    out.execute(Print("\n"))?;
    Ok(())
}

// Returns a line as long as the name
// Something like: ------------------
pub fn get_styled_line(len: usize) -> Vec<StyledContent<String>> {
    let mut ret: Vec<StyledContent<String>> = vec![];
    for _ in 0..len {
        ret.push("-".to_string().white());
    }
    ret
}
