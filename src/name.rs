use std::{env, fs};

use crossterm::style::{Colorize, Styler};

use crate::styled_data::StyledData;

fn get_user_name() -> String {
    env::var("USER")
        .unwrap_or("".to_string())
        .trim()
        .to_string()
}
fn get_host_name() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or("".to_string())
        .trim()
        .to_string()
}

pub fn get_styled_name() -> StyledData {
    let user_cyan = get_user_name().cyan();
    let host_cyan = get_host_name().cyan();

    let user_len = &user_cyan.content().len();
    let host_len = &host_cyan.content().len();

    StyledData::from(
        [user_cyan, "@".to_string().white().bold(), host_cyan].to_vec(),
        user_len + 1 + host_len,
    )
}
