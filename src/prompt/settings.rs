use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag determining if ANSI colors should be printed.
/// Defaults to `true`, but can be disabled via the `NO_COLOR` environment variable.
static COLORS_ENABLED: AtomicBool = AtomicBool::new(true);

/// Checks if ANSI styling is currently enabled globally.
#[inline]
pub fn are_colors_enabled() -> bool {
  COLORS_ENABLED.load(Ordering::Relaxed)
}

/// Manually enable or disable colors globally (useful for testing or user overrides).
pub fn set_colors_enabled(enabled: bool) {
  COLORS_ENABLED.store(enabled, Ordering::Relaxed);
}
