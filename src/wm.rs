use std::convert::TryInto;

use crossterm::style::Colorize;
use x11rb::{
    connection::Connection,
    protocol::xproto::{AtomEnum, ConnectionExt},
};

use crate::styled_data::StyledData;

fn get_wm_name() -> String {
    let conn = x11rb::connect(None).unwrap().0;
    let screen = &conn.setup().roots[0];
    let window = screen.root;

    let wm_check_atom = conn
        .intern_atom(false, b"_NET_SUPPORTING_WM_CHECK")
        .unwrap()
        .reply()
        .unwrap();

    let wm_check_ret = x11rb::protocol::xproto::get_property(
        &conn,
        false,
        window,
        wm_check_atom.atom,
        AtomEnum::NONE,
        0,
        1024,
    )
    .unwrap()
    .reply()
    .unwrap()
    .value;

    let wm_window = u32::from_le_bytes(wm_check_ret.as_slice().try_into().unwrap());

    let net_wm_name_atom = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .unwrap()
        .reply()
        .unwrap();

    let wm_name_ret = x11rb::protocol::xproto::get_property(
        &conn,
        false,
        wm_window,
        net_wm_name_atom.atom,
        AtomEnum::NONE,
        0,
        1024,
    )
    .unwrap()
    .reply()
    .unwrap()
    .value;

    std::str::from_utf8(wm_name_ret.as_slice())
        .unwrap()
        .to_string()
}

pub fn get_styled_wm() -> StyledData {
    let wm_name = get_wm_name();

    StyledData::from(vec!["WM: ".to_string().cyan(), wm_name.white()], 4)
}
