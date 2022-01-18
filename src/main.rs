use std::*;

fn main() {
  loop {
    thread::sleep(time::Duration::from_millis(150));
    let active_window = match win32wrap::get_foreground_window() {
      Some(v) => v,
      _ => {
        // skip if the handle to the foreground window is null
        win32wrap::reset_cursor_clipping();
        continue;
      }
    };
    match win32wrap::get_window_classname(active_window) {
      Some(class_name) => if class_name != "ApplicationFrameWindow" {
        // skip if the class name isn't "ApplicationFrameWindow"
        win32wrap::reset_cursor_clipping();
        continue;
      },
      _ => {
        // skip if failed to the class name of the foreground window
        win32wrap::reset_cursor_clipping();
        println!("[ERROR] Failed to get the name of the class to which the foreground window belongs");
        continue;
      }
    };
    if !win32wrap::get_window_text(active_window).contains("Minecraft") {
      // skip if the title of the foreground window doesn't includes "Minecraft"
      win32wrap::reset_cursor_clipping();
      continue;
    }
    let (x, y) = match win32wrap::get_window_center_pos(active_window) {
      Some(v) => v,
      _ => {
        // skip if failed to get the center posistion of the foreground window
        win32wrap::reset_cursor_clipping();
        println!("[ERROR] Failed to get the center posistion of the foreground window");
        continue;
      }
    };
    match win32wrap::get_cursor_info() {
      Some(info) => if info.flags != 0 {
        // if the cursor is not invisible
        win32wrap::reset_cursor_clipping();
        continue;
      },
      _ => {
        // skip if failed to get information of the cursor
        win32wrap::reset_cursor_clipping();
        println!("[ERROR] Failed to get information about the global cursor");
        continue;
      }
    };
    win32wrap::clip_cursor(x, y, x, y);
  }
}

mod win32wrap {
  use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

  pub fn get_foreground_window() -> Option<HWND> {
    let window_handle = unsafe{GetForegroundWindow()};
    if window_handle.0 == 0 { None } else { Some(window_handle) }
  }

  pub fn clip_cursor(left: i32, top: i32, right: i32, bottom: i32) -> bool {
    unsafe{ClipCursor(&RECT{ left, top, right, bottom })}.0 != 0
  }

  pub fn reset_cursor_clipping() -> bool {
    unsafe{ClipCursor(std::ptr::null())}.0 != 0
  }

  pub fn get_window_classname(window_handle: HWND) -> Option<String> {
    if window_handle.0 == 0 { return None }
    let mut buf = [0u8; 256];
    let length = unsafe{GetClassNameA(
      window_handle,
      PSTR(&mut buf[0]),
      256
    )};
    if length == 0 { return None }
    let mut buf_vec= buf.to_vec();
    buf_vec.resize(length as usize, 0);
    match String::from_utf8(buf_vec) {
      Ok(s) => Some(s),
      _ => None
    }
  }

  pub fn get_window_center_pos(window_handle: HWND) -> Option<(i32, i32)> {
    if window_handle.0 == 0 { return None }
    let mut window_bounding_rect = RECT{ left: 0, top: 0, right: 0, bottom: 0 };
    if unsafe{GetWindowRect(window_handle, &mut window_bounding_rect)}.0 == 0 { 
      return None;
    }
    let mut info = TITLEBARINFO {
      cbSize: std::mem::size_of::<TITLEBARINFO>() as u32,
      rcTitleBar: RECT{ left: 0, top: 0, right: 0, bottom: 0 },
      rgstate: [0; 6]
    };
    if unsafe{GetTitleBarInfo(window_handle, &mut info)}.0 == 0  { 
      return None;
    }
    let titlebar_height = info.rcTitleBar.bottom - info.rcTitleBar.top;
    let x = (window_bounding_rect.left + window_bounding_rect.right) / 2;
    let y =  (window_bounding_rect.top + window_bounding_rect.bottom + titlebar_height) / 2;
    Some((x, y))
  }

  pub fn get_cursor_info() -> Option<CURSORINFO> {
    let mut info = CURSORINFO {
      cbSize: std::mem::size_of::<CURSORINFO>() as u32,
      flags: 0,
      hCursor: HCURSOR(0),
      ptScreenPos: POINT { x: 0, y: 0 }
    };
    if unsafe {GetCursorInfo(&mut info)}.0 == 0 { None } else { Some(info) }
  }

  pub fn get_window_text(window_handle: HWND) -> String {
    if window_handle.0 == 0 { return String::new() }
    let mut inferred_length = unsafe{GetWindowTextLengthA(window_handle)};
    if inferred_length == 0 { return String::new() }
    let mut buf = Vec::new();
    inferred_length += 1;
    buf.resize(inferred_length as usize, 0);
    let length = unsafe{GetWindowTextA(window_handle, PSTR(buf.as_mut_ptr()), inferred_length)};
    buf.resize(length as usize, 0);
    String::from_utf8(buf).unwrap_or(String::new())
  }
}
