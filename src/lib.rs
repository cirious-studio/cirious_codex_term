//! # Cirious Codex Term
//!
//! A high-performance, zero-allocation terminal styling and manipulation library
//! designed as the foundational bedrock for the Cirious ecosystem.
//!
//! This crate provides an intuitive and elegant API for ANSI terminal colors,
//! text styles, cursor movement, and screen clearing.
//!
//! ## Quick Start
//!
//! Simply import the `StyleExt` trait to seamlessly style any type that
//! implements `std::fmt::Display`.
//!
//! ```rust
//! use cirious_codex_term::StyleExt;
//!
//! // Style strings natively
//! println!("{}", "Warning: High memory usage!".yellow().bold().blink());
//!
//! // Style any type seamlessly
//! let age = 25;
//! println!("You are {} years old.", age.green().underline());
//! ```

// Enables docs.rs features to show tags like "Only on Windows"
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Ensures all public items are documented (Essential for crates.io)
#![warn(missing_docs)]
// Prevents accidental unsafe code in the entire crate
#![deny(unsafe_code)]

/// ANSI control structures (Colors, Styles, Cursor, and Screen).
pub mod control;

/// Terminal lifecycle and environment utilities.
pub mod prompt;

/// Asynchronous terminal event handling (Keyboard, Mouse).
pub mod event;

// -----------------------------------------------------------------------------
// PUBLIC API EXPORTS
// -----------------------------------------------------------------------------

// Styling & Control
pub use control::{Color, Cursor, Screen, Style, StyleExt, StyledText};

// Events
pub use event::{poll, read, Event, KeyCode, KeyEvent};

// Terminal Lifecycle
pub use prompt::{init_term, RawModeGuard, TerminalSize};

// Windows-specific utilities
#[cfg(windows)]
pub use prompt::enable_ansi_support;
