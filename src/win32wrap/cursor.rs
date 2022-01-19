
use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

pub fn clip(left: i32, top: i32, right: i32, bottom: i32) -> bool {
  unsafe{ClipCursor(&RECT{ left, top, right, bottom })}.0 != 0
}

pub fn reset_clipping() -> bool {
  unsafe{ClipCursor(std::ptr::null())}.0 != 0
}

pub fn get_info() -> Option<CURSORINFO> {
  let mut info = CURSORINFO {
    cbSize: std::mem::size_of::<CURSORINFO>() as u32,
    flags: 0,
    hCursor: HCURSOR(0),
    ptScreenPos: POINT { x: 0, y: 0 }
  };
  if unsafe {GetCursorInfo(&mut info)}.0 == 0 { None } else { Some(info) }
}
