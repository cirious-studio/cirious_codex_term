#[cfg(windows)]
pub fn enable_ansi_support() -> bool {
  use windows_sys::Win32::Foundation::GetStdHandle;
  use windows_sys::Win32::System::Console::{
    GetConsoleMode, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
  };

  /// Enables ANSI escape code support for the Windows terminal.
  ///
  /// This function is safe to call because it only reads and modifies the console mode.
  ///
  /// # Safety
  /// - The handle is valid and refers to the console output.
  /// - The mode is valid and can be modified.
  ///
  /// # Returns
  /// Returns `true` if ANSI support was enabled successfully, `false` otherwise.
  #[inline]
  #[allow(unsafe_code)]
  unsafe {
    let handle = GetStdHandle(STD_OUTPUT_HANDLE);
    let mut mode = 0;
    if GetConsoleMode(handle, &mut mode) == 0 {
      return false;
    }
    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    SetConsoleMode(handle, mode) != 0
  }
}
