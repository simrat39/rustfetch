use std::fs;

use crossterm::style::Colorize;

use crate::styled_data::StyledData;

pub struct CpuInfo {
    pub name: String,
    pub cores: String,
    pub siblings: String,
}

impl CpuInfo {
    fn new() -> CpuInfo {
        CpuInfo {
            name: "".to_string(),
            cores: "".to_string(),
            siblings: "".to_string(),
        }
    }
}

fn get_raw_cpu_data() -> String {
    fs::read_to_string("/proc/cpuinfo").unwrap()
}

fn parse_line(line: &str) -> &str {
    &line.split(':').into_iter().nth(1).unwrap().trim()
}

fn parse_raw_cpu_data(raw: String) -> CpuInfo {
    let mut info = CpuInfo::new();

    for line in raw.lines().take(20) {
        if line.contains("model name") {
            info.name = parse_line(line).to_string();
            continue;
        } else if line.contains("siblings") {
            info.siblings = parse_line(line).to_string();
            continue;
        } else if line.contains("cpu cores") {
            info.cores = parse_line(line).to_string();
            break;
        }
    }
    info
}

pub fn get_styled_cpu() -> StyledData {
    let cpu_info = parse_raw_cpu_data(get_raw_cpu_data());

    let formated_string = format!(
        "{} ({}c {}t)",
        cpu_info.name, cpu_info.cores, cpu_info.siblings
    );

    let len = formated_string.len();

    StyledData::from(
        vec!["CPU: ".to_string().cyan(), formated_string.white()],
        len + 5,
    )
}

#[cfg(test)]
mod tests {
    use super::parse_line;
    #[test]
    fn line_parse_test() {
        let line = "This is a: Test";
        assert_eq!(parse_line(line), "Test");
    }
}
