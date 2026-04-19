//! Wrapped interval abstract domain for Solana eBPF.
//!
//! Based on the paper "A Wrapped Interval Arithmetic" by Jorge A. Navas et al.
//!
//! This crate is `no_std` compatible. Enable the `std` feature for debug printing.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::ptr_as_ptr)]

extern crate alloc;

mod wrapped_interval;

pub use wrapped_interval::{BaseRange, WrappedRange};
