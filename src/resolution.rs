use crossterm::style::Colorize;
use x11rb::{
    connection::Connection,
    protocol::{randr::MonitorInfo, xproto::Window},
    rust_connection::DefaultStream,
};

use crate::styled_data::StyledData;

fn get_all_monitors() -> Vec<MonitorInfo> {
    let conn = x11rb::connect(None).unwrap();
    let rust_conn = x11rb::rust_connection::RustConnection::<DefaultStream>::connect(None).unwrap();
    let screens = &conn.0.setup().roots;
    let root: Window = screens[0].root;

    let req = x11rb::protocol::randr::GetMonitorsRequest {
        window: root,
        get_active: true,
    };

    req.send(&rust_conn.0).unwrap().reply().unwrap().monitors
}

fn get_res_from_monitor(monitor: &MonitorInfo) -> String {
    format!("{}x{}", monitor.width, monitor.height)
}

fn get_all_resolutions(monitors: &[MonitorInfo]) -> String {
    let mut ret = String::new();
    for (index, monitor) in monitors.iter().enumerate() {
        ret.push_str(get_res_from_monitor(monitor).as_str());
        if index + 1 != monitors.len() {
            ret.push(',');
        }
        ret.push(' ');
    }
    ret.trim().to_string()
}

pub fn get_styled_resolution() -> StyledData {
    let monitors = get_all_monitors();
    let resolutions = get_all_resolutions(&monitors);
    let len = resolutions.len();

    StyledData::from(
        vec!["Resolution: ".to_string().cyan(), resolutions.white()],
        len + 12,
    )
}
