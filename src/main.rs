use std::io::stdout;

use rustfetch::{
    de::get_styled_de, get_styled_line, kernel::get_styled_kernel, memory::get_styled_memory,
    name::get_styled_name, os::get_styled_os, shell::get_styled_shell, wm::get_styled_wm,
};
use rustfetch::{print_styled_text, uptime::get_styled_uptime};

fn main() {
    let mut out = stdout();
    let padding: usize = 2;

    let styled_os = get_styled_os();
    let styled_kernel = get_styled_kernel();
    let styled_name = get_styled_name();
    let styled_uptime = get_styled_uptime();
    let styled_de = get_styled_de();
    let styled_wm = get_styled_wm();
    let styled_shell = get_styled_shell();
    let styled_memory = get_styled_memory();

    println!();

    print_styled_text(&mut out, &styled_name.styles, padding).unwrap();
    print_styled_text(&mut out, &get_styled_line(styled_name.len), padding).unwrap();

    print_styled_text(&mut out, &styled_os.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_kernel.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_uptime.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_de.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_wm.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_shell.styles, padding).unwrap();
    print_styled_text(&mut out, &styled_memory.styles, padding).unwrap();

    println!();
}
