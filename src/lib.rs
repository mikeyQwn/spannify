//! # Spannify
//!
//! This crate provides functionality to produce nice-looking graphs that represent your
//! callstack. It is designed to help developers trace and understand the execution flow of their
//! programs, making it easier to debug and optimize their code.
//!
//! ## Overview
//!
//! The core functionality of this crate revolves around creating spans that represent different
//! sections of your code. These spans are organized hierarchically, and the crate generates
//! messages to visualize when you enter and exit each span. The generated messages can be
//! customized to suit your preferences using a variety of configuration options.
//!
//! ## Modules
//!
//! - [`config`]: Contains the configuration structures and options for customizing the appearance
//!   and behavior of the callstack visualization.
//! - [`core`]: Contains the core functionality for managing spans, generating messages, and
//!   handling the callstack visualization logic.
//!
//! ## Example
//!
//! ```rust
//!  use spannify::core::StdoutSpanner;
//!
//!  fn fib(s: &StdoutSpanner, x: usize) -> usize {
//!      let _span = s.enter_span(format!("fib({})", x).as_ref());
//!      match x {
//!          0 => 0,
//!          1 | 2 => 1,
//!          _ => fib(s, x - 1) + fib(s, x - 2),
//!      }
//!  }

//!  
//!  fn main() {
//!      let spanner = StdoutSpanner::new();

//!      let _ = fib(&spanner, 5);
//!  }
//!
//!  ```
//!
//! ### Output
//!
//!  ```text
//! ┌fib(5) entered
//! |  fib(4) entered
//! |   ┌fib(3) entered
//! |   ┆  fib(2) entered
//! |   ┆  fib(2) dropped
//! |   ┆  fib(1) entered
//! |   ┆  fib(1) dropped
//! |   └fib(3) dropped
//! |   ┌fib(2) entered
//! |   └fib(2) dropped
//! |  fib(4) dropped
//! |  fib(3) entered
//! |   ┌fib(2) entered
//! |   └fib(2) dropped
//! |   ┌fib(1) entered
//! |   └fib(1) dropped
//! |  fib(3) dropped
//! └fib(5) dropped
//! ```
//!
//! ## Usage
//!
//! To use this crate, you typically start by creating a `spannify` instance with a desired writer
//! and configuration. Then, you create spans by calling the `Span::enter` method, which tracks the
//! entry and exit points of different sections of your code.
//!
//! ```rust
//! use spannify::{config::Config, core::{StdoutSpanner}};
//! use std::io::stdout;
//!
//! // Create a configuration
//! let config = Config::default();
//!
//! // Create a spannify
//! let spanner = StdoutSpanner::new().with_config(config);
//!
//! // Create a span
//! {
//!     let _span = spanner.enter_span("main");
//!     // Your code here...
//! }
//! // The span is automatically dropped here, and the exit message is generated
//! ```
//!
//! ## Configuration
//!
//! The [`config`] module provides various options to customize the appearance and behavior of the
//! callstack visualization. This includes settings for indentation, depth display, and message
//! formatting. You can create a custom configuration by modifying the default values.
//!
//! ```rust
//! use spannify::config::Config;
//!
//! let mut config = Config::default();
//! config.tabwidth = 4;
//! config.skip = 2;
//! config.depthmap = |depth| if depth % 2 == 0 { '|' } else { '^' };
//!
//! // Use this configuration when creating a spannify
//! ```
//!
//! ## License
//!
//! This crate is licensed under the MIT License. See the LICENSE file for more details.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please open an issue or submit a pull request

#![doc(html_root_url = "https://docs.rs/spannify/latest")]

pub mod config;
pub mod core;
