//! MinerApp
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

pub mod application;
pub mod block;
pub mod commands;
pub mod config;
pub mod delay;
pub mod error;
pub mod prelude;
pub mod submit_tx;
pub mod test_tx_swarm;
pub mod backlog;
pub mod node_keys;
pub mod keygen;
pub mod account;

