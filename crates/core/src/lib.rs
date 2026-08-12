//! Nexivora core: shared types, errors, and utilities.
//!
//! This crate provides the foundational data types used across all
//! Nexivora engines: errors, document identifiers, cell references,
//! units, and text utilities.

pub mod error;
pub mod id;
pub mod text;
pub mod types;
pub mod units;

pub use error::{Error, Result};
pub use types::*;
