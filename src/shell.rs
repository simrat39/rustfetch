use std::env;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_shell_path() -> String {
    env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
}

pub fn get_styled_shell() -> StyledData {
    let shell = get_shell_path();
    let len = shell.len();

    StyledData::from(vec!["Shell: ".to_string().cyan(), shell.white()], len + 7)
}
