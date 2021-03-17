use std::fs;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_raw_mem_data() -> String {
    fs::read_to_string("/proc/meminfo").expect("An error occured while reading /proc/meminfo")
}

fn get_total_memory(raw: &str) -> usize {
    return raw
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
}

fn get_available_memory(raw: &str) -> usize {
    return raw
        .lines()
        .nth(2)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
}

fn get_used_memory(total: usize, availabile: usize) -> usize {
    total - availabile
}

fn kb_to_mb(kb: usize) -> usize {
    kb / 1024
}

pub fn get_styled_memory() -> StyledData {
    let raw = get_raw_mem_data();
    let total = kb_to_mb(get_total_memory(&raw));
    let available = kb_to_mb(get_available_memory(&raw));
    let free = get_used_memory(total, available);

    let total_str = total.to_string();
    let free_str = free.to_string();

    let len = total_str.len() + free_str.len();

    StyledData::from(
        vec![
            "Memory: ".to_string().cyan(),
            free_str.white(),
            "_MB / ".to_string().white(),
            total_str.white(),
            "_MB".to_string().white(),
        ],
        len + 11,
    )
}
