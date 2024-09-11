//! [![github]](https://github.com/raeisimv/millisecond)&ensp;[![crates-io]](https://crates.io/crates/millisecond)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! A better way to format and display time. This crate converts 33023448000 to 1y 17d 5h 10m 48s
pub use formatter::MillisecondPart;
pub use splitter::Millisecond;

mod formatter;
mod splitter;
