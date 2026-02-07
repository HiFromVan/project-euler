//! # Project Euler Solutions
//!
//! This crate contains solutions to [Project Euler](https://projecteuler.net/) problems.
//!
//! Each problem is implemented as a separate binary in the `problems/` directory.
//!
//! ## Usage
//!
//! Run a specific problem:
//! ```bash
//! cargo run --bin 001
//! ```
//!
//! Run tests for a problem:
//! ```bash
//! cargo test --bin 001
//! ```
//!
//! Generate documentation with rendered math formulas:
//! ```bash
//! cargo doc --open
//! ```

/// Common mathematical utilities for Project Euler problems
pub mod math {
    /// Check if a number is prime
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let limit = (n as f64).sqrt() as u64;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// Calculate greatest common divisor using Euclidean algorithm
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Calculate least common multiple
    pub fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }
}
