use std::env;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_de() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or(" ".to_string())
        .to_string()
}

pub fn get_styled_de() -> StyledData {
    let de = get_de();
    let len = de.len();

    StyledData::from(vec!["DE: ".to_string().cyan(), de.white()], len + 4)
}
