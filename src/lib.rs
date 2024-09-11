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
//! - [`level`]: Contains the levels of spans, which determines if the span should be outputted or not
//!
//! ## Example
//!
//! ```rust
//!  use spannify::core::StdoutSpanner;
//!
//!  fn fib(s: &StdoutSpanner, x: usize) -> usize {
//!      let _span = s.enter_span(format!("fib({x})").as_ref());
//!      match x {
//!          0 => 0,
//!          1 | 2 => 1,
//!          _ => fib(s, x - 1) + fib(s, x - 2),
//!      }
//!  }
//!
//!  
//!  fn main() {
//!      let spanner = StdoutSpanner::new();
//!      let _ = fib(&spanner, 5);
//!  }
//!
//!  ```
//! ### Output
//!
//!  ```text
//!┌fib(5)
//!|  fib(4)
//!|   ┌fib(3)
//!|   ┆  fib(2)
//!|   ┆  fib(2)
//!|   ┆  fib(1)
//!|   ┆  fib(1)
//!|   └fib(3)
//!|   ┌fib(2)
//!|   └fib(2)
//!|  fib(4)
//!|  fib(3)
//!|   ┌fib(2)
//!|   └fib(2)
//!|   ┌fib(1)
//!|   └fib(1)
//!|  fib(3)
//!└fib(5)
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
#![deny(unsafe_code)]
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::similar_names,
    clippy::suspicious_operation_groupings,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

pub mod config;
pub mod core;
pub mod level;
