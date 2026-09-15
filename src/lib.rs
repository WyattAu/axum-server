#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Axum server utilities — shutdown, CORS, request ID, health routes, middleware stack.
//!
//! Provides reusable building blocks for Axum HTTP servers:
//! - [`cors`] — CORS layer presets
//! - [`request_id`] — Request ID middleware
//! - [`health`] — Health check routes
//! - [`middleware`] — Common middleware stack
//! - [`server`] — One-line server builder

pub mod cors;
pub mod request_id;
pub mod health;
pub mod middleware;
pub mod server;

pub use shutdown_kit::shutdown_signal;
