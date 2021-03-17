use std::fs;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_raw_uptime() -> usize {
    let s = fs::read_to_string("/proc/uptime")
        .unwrap_or_else(|_| "0".to_string())
        .split(' ')
        .into_iter()
        .next()
        .unwrap_or("0")
        .to_string();

    s.parse::<f64>().unwrap_or(0_f64) as usize
}

fn parse_seconds(seconds: usize) -> String {
    let mut ret = String::new();

    let hours = seconds / 60 / 60;
    let mut remainder = seconds % (60 * 60);

    let minutes = remainder / 60;
    remainder %= 60;

    let left_seconds = remainder;

    if hours >= 1 {
        ret.push_str((hours.to_string() + &"h ".to_string()).as_str());
    }
    if minutes >= 1 {
        ret.push_str((minutes.to_string() + &"m ".to_string()).as_str());
    }
    if left_seconds >= 1 {
        ret.push_str((left_seconds.to_string() + &"s ".to_string()).as_str());
    }
    ret
}

pub fn get_styled_uptime() -> StyledData {
    let u = parse_seconds(get_raw_uptime());
    let len = u.len();

    let styles = vec![
        "Uptime".to_string().cyan(),
        ": ".to_string().cyan(),
        u.white(),
    ];

    StyledData::from(styles, 8 + len)
}
