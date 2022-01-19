
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
