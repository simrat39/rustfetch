use std::{
    error::Error,
    io::{stdout, Stdout},
};

use cpu::get_styled_cpu;
use crossterm::{
    style::{self, Colorize, Print, StyledContent},
    ExecutableCommand, QueueableCommand,
};
use de::get_styled_de;
use kernel::get_styled_kernel;
use memory::get_styled_memory;
use name::get_styled_name;
use os::get_styled_os;
use shell::get_styled_shell;
use uptime::get_styled_uptime;
use wm::get_styled_wm;

pub mod cpu;
pub mod de;
pub mod kernel;
pub mod memory;
pub mod name;
pub mod os;
pub mod shell;
pub mod styled_data;
pub mod uptime;
pub mod wm;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut out = stdout();
    let padding: usize = 2;

    let styled_os = get_styled_os();
    let styled_kernel = get_styled_kernel();
    let styled_cpu = get_styled_cpu();
    let styled_name = get_styled_name();
    let styled_uptime = get_styled_uptime();
    let styled_de = get_styled_de();
    let styled_wm = get_styled_wm();
    let styled_shell = get_styled_shell();
    let styled_memory = get_styled_memory();

    println!();

    print_styled_text(&mut out, &styled_name.styles, padding)?;
    print_styled_text(&mut out, &get_styled_line(styled_name.len), padding)?;

    print_styled_text(&mut out, &styled_os.styles, padding)?;
    print_styled_text(&mut out, &styled_kernel.styles, padding)?;
    print_styled_text(&mut out, &styled_cpu.styles, padding)?;
    print_styled_text(&mut out, &styled_uptime.styles, padding)?;
    print_styled_text(&mut out, &styled_de.styles, padding)?;
    print_styled_text(&mut out, &styled_wm.styles, padding)?;
    print_styled_text(&mut out, &styled_shell.styles, padding)?;
    print_styled_text(&mut out, &styled_memory.styles, padding)?;

    println!();
    Ok(())
}

fn print_styled_text(
    out: &mut Stdout,
    vec: &[StyledContent<String>],
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
fn get_styled_line(len: usize) -> Vec<StyledContent<String>> {
    let mut ret: Vec<StyledContent<String>> = vec![];
    for _ in 0..len {
        ret.push("-".to_string().white());
    }
    ret
}
