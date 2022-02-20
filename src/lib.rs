#![feature(crate_visibility_modifier, lint_reasons, path_try_exists, try_blocks)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::correctness,
    clippy::style,
    clippy::all,
    clippy::suspicious,
    //clippy::restriction,
    clippy::perf,
    clippy::no_effect,
    coherence_leak_check,
    unsafe_op_in_unsafe_fn,
    rustdoc::all,
    deprecated_in_future,
    future_incompatible,
    nonstandard_style,
    rust_2021_compatibility,
    unknown_lints
)]
#![allow(dead_code, clippy::module_name_repetitions)]

mod cli;
crate mod core;
mod dynamic_libs;
mod functions;
crate mod util;

pub use crate::core::types::{ComputeFunction, ComputeRequest, ComputeResponse};
pub use async_trait::async_trait;
pub use serde_json::{json, Value as JsonValue};

// Dumb change.
