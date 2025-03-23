//! A library for AVL trees and related operations

pub mod core;
pub mod map;
pub mod set;
pub mod traversal;

#[cfg(feature = "serde")]
mod serde_impl;
