#![warn(clippy::restriction, clippy::perf)]
#![allow(
    clippy::implicit_return,
    clippy::missing_inline_in_public_items
)]
pub mod position;
pub mod selection;
pub mod document;
pub mod buffer;
pub mod tab;
pub mod diagnostics;
pub mod errors;
