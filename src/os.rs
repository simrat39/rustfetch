use std::fs;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_raw_os_data() -> String {
    fs::read_to_string("/etc/os-release").unwrap()
}

fn get_pretty_name(raw: &String) -> String {
    for i in raw.lines() {
        if i.contains("PRETTY_NAME") {
            return i
                .split("=")
                .nth(1)
                .unwrap()
                .split("\"")
                .nth(1)
                .unwrap()
                .to_string();
        }
    }
    return "".to_string();
}

pub fn get_styled_os() -> StyledData {
    let raw = get_raw_os_data();
    let pretty = get_pretty_name(&raw);

    let len = pretty.len();

    StyledData::from(vec!["OS: ".to_string().cyan(), pretty.white()], len + 4)
}
