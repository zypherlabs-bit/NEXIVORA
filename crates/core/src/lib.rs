//! Nexivora core: shared types, errors, and utilities.
//!
//! This crate provides the foundational data types used across all
//! Nexivora engines: errors, document identifiers, cell references,
//! units, and text utilities.

pub mod error;
pub mod types;
pub mod units;
pub mod text;
pub mod id;

pub use error::{Error, Result};
pub use types::*;