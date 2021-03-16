use std::fs;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

fn get_kernel_release() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or("".to_string())
        .trim()
        .to_string()
}

pub fn get_styled_kernel() -> StyledData {
    let version = get_kernel_release();
    let len = version.len();

    StyledData::from(
        vec!["Kernel: ".to_string().cyan(), version.white()],
        len + 8,
    )
}
