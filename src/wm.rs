use std::{ffi::CString, ptr};

use crossterm::style::Colorize;
use x11_dl::xlib;

use crate::styled_data::StyledData;

fn get_wm_name() -> String {
    let mut wm: String = String::new();
    unsafe {
        let xlib = xlib::Xlib::open().unwrap();
        let display = (xlib.XOpenDisplay)(ptr::null());

        let screen = (xlib.XDefaultScreen)(display);
        let root = (xlib.XRootWindow)(display, screen);

        let net_wm_name = CString::new("_NET_WM_NAME").unwrap();
        let utf_8_str = CString::new("UTF8_STRING").unwrap();

        let netwmname = (xlib.XInternAtom)(display, net_wm_name.as_ptr(), 0);

        let utf8string = (xlib.XInternAtom)(display, utf_8_str.as_ptr(), 0);

        let mut real: u64 = 0;
        let mut format: i32 = 0;
        let mut n: u64 = 0;
        let mut extra: u64 = 0;
        let mut data: *mut u8 = &mut 0;

        (xlib.XGetWindowProperty)(
            display,
            root,
            netwmname,
            0,
            32,
            0,
            utf8string,
            &mut real,
            &mut format,
            &mut n,
            &mut extra,
            &mut data,
        );

        let u8_slice = core::slice::from_raw_parts(data, n as usize);
        let name = std::str::from_utf8(u8_slice).unwrap().to_string();

        wm.push_str(&name);
    }
    wm
}

pub fn get_styled_wm() -> StyledData {
    let wm_name = get_wm_name();
    let len = wm_name.len();

    StyledData::from(vec!["WM: ".to_string().cyan(), wm_name.white()], len + 4)
}
