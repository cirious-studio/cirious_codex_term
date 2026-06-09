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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// ANSI control structures
pub mod control;

/// Prompt utilities for terminal input and output.
pub mod prompt;

#[cfg(windows)]
pub use prompt::enable_ansi_support;
pub use prompt::{init_term, RawModeGuard, TerminalSize};

pub use control::{Color, Cursor, Screen, Style, StyleExt, StyledText};
