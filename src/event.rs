use std::{
  io::{self, Read},
  time::Duration,
};

/// Represents terminal events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
  /// A key event.
  Key(KeyEvent),
}

/// Represents keyboard events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
  /// The key code.
  pub code: KeyCode,
}

/// Represents keyboard key codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
  /// Character key.
  Char(char),
  /// Enter key.
  Enter,
  /// Backspace key.
  Backspace,
  /// Escape key.
  Esc,
  /// Up arrow key.
  Up,
  /// Down arrow key.
  Down,
  /// Left arrow key.
  Left,
  /// Right arrow key.
  Right,
  /// Unknown key.
  Unknown,
}

// -----------------------------------------------------------------------------
// POLLING ENGINE
// -----------------------------------------------------------------------------

/// Checks if there is an event waiting to be read.
/// Returns `true` if input is available, or `false` if the timeout expires.
///
/// # Errors
///
/// Returns an `io::Result` if an error occurs while polling for an event.
///
pub fn poll(timeout: Duration) -> io::Result<bool> {
  #[cfg(unix)]
  {
    use libc::{poll, pollfd, POLLIN, STDIN_FILENO};

    let mut pfd = pollfd {
      fd: STDIN_FILENO,
      events: POLLIN,
      revents: 0,
    };

    let timeout_ms = i32::try_from(timeout.as_millis())
      .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Timeout too large"))?;

    #[allow(unsafe_code)]
    // SAFETY: poll is a safe system call when provided with a valid pollfd struct.
    let ret = unsafe { poll(&raw mut pfd, 1, timeout_ms) };

    if ret < 0 {
      Err(io::Error::last_os_error())
    } else {
      Ok(ret > 0)
    }
  }

  #[cfg(windows)]
  {
    use windows_sys::Win32::Foundation::{WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};
    use windows_sys::Win32::System::Threading::WaitForSingleObject;

    let timeout_ms = timeout.as_millis() as u32;

    #[allow(unsafe_code)]
    // SAFETY: We are getting the standard input handle and waiting on it.
    let res = unsafe {
      let handle = GetStdHandle(STD_INPUT_HANDLE);
      WaitForSingleObject(handle, timeout_ms)
    };

    match res {
      WAIT_OBJECT_0 => Ok(true),
      WAIT_TIMEOUT => Ok(false),
      _ => Err(io::Error::last_os_error()),
    }
  }
}

// -----------------------------------------------------------------------------
// EVENT PARSER
// -----------------------------------------------------------------------------

/// Reads an event from `stdin`.
/// This function blocks, so it should only be called AFTER `poll()` returns `true`.
///
/// # Errors
///
/// Returns an `io::Result` if an error occurs while reading from `stdin`.
///
pub fn read() -> io::Result<Event> {
  let mut stdin = io::stdin();
  let mut buffer = [0u8; 1];

  // Read exactly 1 byte
  stdin.read_exact(&mut buffer)?;

  match buffer[0] {
    b'\x1b' => {
      let mut seq = [0u8; 2];
      // Try to read 2 more bytes (ANSI escape sequence)
      if stdin.read_exact(&mut seq).is_err() {
        return Ok(Event::Key(KeyEvent { code: KeyCode::Esc }));
      }

      if seq[0] == b'[' {
        match seq[1] {
          b'A' => Ok(Event::Key(KeyEvent { code: KeyCode::Up })),
          b'B' => Ok(Event::Key(KeyEvent { code: KeyCode::Down })),
          b'C' => Ok(Event::Key(KeyEvent { code: KeyCode::Right })),
          b'D' => Ok(Event::Key(KeyEvent { code: KeyCode::Left })),
          _ => Ok(Event::Key(KeyEvent { code: KeyCode::Unknown })),
        }
      } else {
        Ok(Event::Key(KeyEvent { code: KeyCode::Esc }))
      }
    }
    b'\n' | b'\r' => Ok(Event::Key(KeyEvent { code: KeyCode::Enter })),
    127 | 8 => Ok(Event::Key(KeyEvent {
      code: KeyCode::Backspace,
    })),
    c => Ok(Event::Key(KeyEvent {
      code: KeyCode::Char(c as char),
    })),
  }
}
