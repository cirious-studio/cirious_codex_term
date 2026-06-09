/// Cross-platform terminal abstractions.
pub mod terminal;

/// Raw mode toggle for terminal.
pub mod raw;

/// Terminal application settings.
pub mod settings;

/// Windows specific terminal prompt utilities.
#[cfg(windows)]
pub mod windows;

pub use raw::RawModeGuard;
pub use settings::{are_colors_enabled, set_colors_enabled};
pub use terminal::TerminalSize;

#[cfg(windows)]
pub use windows::enable_ansi_support;

use std::{io::IsTerminal, sync::Once};

/// Init static once for windows support
static INIT: Once = Once::new();

/// initializes the terminal
pub fn init_term() {
  INIT.call_once(|| {
    // 1. Check for the NO_COLOR standard
    let no_color_env = std::env::var("NO_COLOR").is_ok_and(|v| v.parse::<bool>().unwrap_or(false));

    // 2. Check if stdout is actually a terminal
    let is_tty = std::io::stdout().is_terminal();

    if no_color_env || !is_tty {
      set_colors_enabled(false);
    }
    #[cfg(windows)]
    {
      // Only try to enable Windows ANSI if colors aren't already disabled
      if settings::are_colors_enabled() {
        let _ = crate::windows::enable_ansi_support();
      }
    }
  });
}
