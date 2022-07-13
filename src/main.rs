use std::*;
mod win32wrap;

fn main() {
  use win32wrap::window::HWNDExtention;

  thread::spawn(|| loop {
    thread::sleep(time::Duration::from_millis(150));
    let foreground_window = match win32wrap::window::get_foreground_window() {
      Some(v) => v,
      None => {
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
      None => {
        // if failed to get the center posistion of the window
        win32wrap::cursor::reset_clipping();
        println!("[ERROR] Failed to get the center posistion of the foreground window");
        continue;
      }
    };
    match win32wrap::cursor::get_flags() {
      Some(flags) => if flags != 0 {
        // if the flags is not 0
        // 0: hidden
        // 1: showing
        // 2: suppressed
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-cursorinfo
        win32wrap::cursor::reset_clipping();
        continue;
      },
      None => {
        // if failed to get information of the cursor
        win32wrap::cursor::reset_clipping();
        println!("[ERROR] Failed to get information about the global cursor");
        continue;
      }
    };
    win32wrap::cursor::clip(x, y, x, y);
  });
  println!("[INFO] Enter .exit to exit");
  let mut input = String::new();
  loop {
    input.clear();
    io::stdin().read_line(&mut input).expect("Error: Failed on 'read_line()'.");
    if input.trim() == ".exit" { break; }
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
