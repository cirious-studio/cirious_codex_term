use std::io;

/// Guard that enables terminal raw mode while alive and restores the
/// previous terminal configuration when dropped.
#[derive(Debug)]
pub struct RawModeGuard {
  #[cfg(unix)]
  original_termios: libc::termios,
}

impl RawModeGuard {
  /// Enables raw mode for the current terminal.
  ///
  /// Raw mode disables canonical input processing and character echoing,
  /// allowing key presses to be read immediately without requiring Enter.
  ///
  /// # Errors
  ///
  /// Returns an error if the terminal attributes cannot be queried or
  /// updated through the operating system APIs.
  pub fn enable() -> io::Result<Self> {
    #[cfg(unix)]
    {
      use libc::{tcgetattr, tcsetattr, termios, TCSAFLUSH};
      use std::mem::MaybeUninit;

      let mut original = MaybeUninit::<termios>::uninit();

      // SAFETY:
      // - `original.as_mut_ptr()` points to valid writable memory for a
      //   `termios` structure.
      // - The memory remains alive for the duration of the call.
      // - `tcgetattr` initializes the structure on success.
      #[allow(unsafe_code)]
      if unsafe { tcgetattr(libc::STDIN_FILENO, original.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
      }

      // SAFETY:
      // - The preceding `tcgetattr` call returned success.
      // - POSIX guarantees that the provided `termios` structure is fully
      //   initialized on success.
      #[allow(unsafe_code)]
      let original = unsafe { original.assume_init() };
      let mut raw = original;

      // Disable canonical mode and echo.
      raw.c_lflag &= !(libc::ICANON | libc::ECHO);

      // SAFETY:
      // - `&raw` is a valid pointer to a properly initialized `termios`.
      // - The pointer remains valid for the duration of the call.
      // - `STDIN_FILENO` refers to the terminal whose settings were
      //   previously retrieved.
      #[allow(unsafe_code)]
      if unsafe { tcsetattr(libc::STDIN_FILENO, TCSAFLUSH, &raw const raw) } != 0 {
        return Err(io::Error::last_os_error());
      }

      Ok(Self {
        original_termios: original,
      })
    }

    #[cfg(windows)]
    {
      // Windows implementation would use GetConsoleMode and SetConsoleMode
      // to unset ENABLE_LINE_INPUT and ENABLE_ECHO_INPUT.
      Ok(Self {})
    }
  }
}

impl Drop for RawModeGuard {
  fn drop(&mut self) {
    #[cfg(unix)]
    {
      // SAFETY:
      // - `original_termios` was obtained from a successful `tcgetattr` call.
      // - The pointer remains valid for the duration of the call.
      // - Restoring terminal attributes is permitted for this file descriptor.
      #[allow(unsafe_code)]
      unsafe {
        let _ = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw const self.original_termios);
      }
    }
  }
}
