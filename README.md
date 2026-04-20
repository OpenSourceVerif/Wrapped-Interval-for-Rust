# WrappedInterval-for-Rust

A `no_std`-compatible **Wrapped Interval** abstract domain implementation in pure Rust, designed for static analysis of eBPF programs (Solana SBF).

## Reference
- "Interval Analysis and Machine Arithmetic: Why Signedness Ignorance Is Bliss". TOPLAS2015. https://dl.acm.org/doi/10.1145/2651360
- "Program Analysis Combining Generalized Bit-Level and Word-Level Abstractions". ISSTA2025. https://dl.acm.org/doi/10.1145/3728905

## Overview

Wrapped intervals extend traditional interval arithmetic to handle modular (wrapping) semantics common in fixed-width integer operations. This domain represents ranges `[lb, ub]` where the interval may wrap around the modulus boundary, enabling precise tracking of bit-width-aware value ranges.

## Features

- `no_std` compatible (no standard library dependency)
- `#[cfg(feature = "std")]` for opt-in standard library features
- Full arithmetic operations: add, sub, mul, div, rem
- Bitwise operations: and, or
- Byte swap: bswap16, bswap32, bswap64
- Signed/unsigned split for precise division
- Type-safe abstract domain with subset, meet, and join operations
- LTO-optimized release profile

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wrapped_interval = { git = "https://github.com/OpenSourceVerif/WrappedInterval-for-Rust" }
```

Or from [crates.io](https://crates.io) once published:

```toml
[dependencies]
wrapped_interval = "0.1"
```

## Usage

```rust
use wrapped_interval::WrappedRange;

// Creation
let r = WrappedRange::new_constant(100, 64);
let r = WrappedRange::top(64);     // completely unknown
let r = WrappedRange::bottom(64); // impossible value
let r = WrappedRange::new_bounds(0, 255, 64);

// Arithmetic
let sum = a.add(&b);
let diff = a.sub(&b);
let prod = a.mul(&b);
let quot = a.udiv(&b);
let rem = a.urem(&b);

// Byte swap
let swapped = r.bswap32();

// Domain operations
let joined = a.or(&b);
let meets = a.exact_meet(&b, &mut out);
```

## Integration with Solana SBF

This crate powers the value analysis in the Solana eBPF verifier and interpreter. The `WrappedRange` domain tracks register values with wrapped interval precision, complementing the Tnum domain for reduced product analysis.
