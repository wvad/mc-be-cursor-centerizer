use std::*;

fn main() {
  use win32wrap::window::HWNDExtention;
  loop {
    thread::sleep(time::Duration::from_millis(150));
    let foreground_window = match win32wrap::window::get_foreground_window() {
      Some(v) => v,
      _ => {
        // if the handle to the foreground window is null
        win32wrap::cursor::reset_clipping();
        continue;
      }
    };
    if foreground_window.get_classname() != "ApplicationFrameWindow" {
      // if the window classname isn't "ApplicationFrameWindow"
      win32wrap::cursor::reset_clipping();
      continue;
    }
    if !foreground_window.get_title().contains("Minecraft") {
      // if the window title doesn't includes "Minecraft"
      win32wrap::cursor::reset_clipping();
      continue;
    }
    let (x, y) = match get_window_center_pos(foreground_window) {
      Some(v) => v,
      _ => {
        // if failed to get the center posistion of the window
        win32wrap::cursor::reset_clipping();
        println!("[ERROR] Failed to get the center posistion of the foreground window");
        continue;
      }
    };
    match win32wrap::cursor::get_info() {
      Some(info) => if info.flags != 0 {
        // if the flags is not 0
        // 0: hidden
        // 1: showing
        // 2: suppressed
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-cursorinfo
        win32wrap::cursor::reset_clipping();
        continue;
      },
      _ => {
        // if failed to get information of the cursor
        win32wrap::cursor::reset_clipping();
        println!("[ERROR] Failed to get information about the global cursor");
        continue;
      }
    };
    win32wrap::cursor::clip(x, y, x, y);
  }
}

pub fn get_window_center_pos(window_handle: windows::Win32::Foundation::HWND) -> Option<(i32, i32)> {
  use win32wrap::window::HWNDExtention;
  let window_bounding_rect = match window_handle.get_rect() {
    Some(v) => v,
    _ => return None
  };
  let titlebar_info = match window_handle.get_titlebar_info() {
    Some(v) => v,
    _ => return None
  };
  let titlebar_height = titlebar_info.rcTitleBar.bottom - titlebar_info.rcTitleBar.top;
  let x = (window_bounding_rect.left + window_bounding_rect.right) / 2;
  let y =  (window_bounding_rect.top + window_bounding_rect.bottom + titlebar_height) / 2;
  Some((x, y))
}

mod win32wrap {
  pub mod cursor {
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
  }

  pub mod window {
    use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

    pub fn get_foreground_window() -> Option<HWND> {
      let window_handle = unsafe{GetForegroundWindow()};
      if window_handle.0 == 0 { None } else { Some(window_handle) }
    }

    pub trait HWNDExtention {
      fn get_classname(self) -> String;
      fn get_title(self) -> String;
      fn get_rect(self) -> Option<RECT>;
      fn get_titlebar_info(self) -> Option<TITLEBARINFO>;
    }

    impl HWNDExtention for windows::Win32::Foundation::HWND {
      fn get_classname(self) -> String {
        if self.0 == 0 { return String::new() }
        let mut buf = [0u8; 256];
        let length = unsafe{GetClassNameA(
          self,
          PSTR(&mut buf[0]),
          256
        )};
        if length == 0 { return String::new() }
        let mut buf_vec= buf.to_vec();
        buf_vec.resize(length as usize, 0);
        String::from_utf8(buf_vec).unwrap_or(String::new())
      }

      fn get_title(self) -> String {
        if self.0 == 0 { return String::new() }
        let mut inferred_length = unsafe{GetWindowTextLengthA(self)};
        if inferred_length == 0 { return String::new() }
        let mut buf = Vec::new();
        inferred_length += 1;
        buf.resize(inferred_length as usize, 0);
        let length = unsafe{GetWindowTextA(self, PSTR(buf.as_mut_ptr()), inferred_length)};
        buf.resize(length as usize, 0);
        String::from_utf8(buf).unwrap_or(String::new())
      }

      fn get_rect(self) -> Option<RECT> {
        if self.0 == 0 { return None }
        let mut rect = RECT{ left: 0, top: 0, right: 0, bottom: 0 };
        if unsafe{GetWindowRect(self, &mut rect)}.0 == 0 { None } else { Some(rect) }
      }

      fn get_titlebar_info(self) -> Option<TITLEBARINFO> {
        if self.0 == 0 { return None }
        let mut info = TITLEBARINFO {
          cbSize: std::mem::size_of::<TITLEBARINFO>() as u32,
          rcTitleBar: RECT{ left: 0, top: 0, right: 0, bottom: 0 },
          rgstate: [0; 6]
        };
        if unsafe{GetTitleBarInfo(self, &mut info)}.0 == 0 { None } else { Some(info) }
      }
    } 
  }
}
