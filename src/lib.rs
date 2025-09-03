//! [![github]](https://github.com/raeisimv/millisecond)&ensp;[![crates-io]](https://crates.io/crates/millisecond)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! A better way to format and display time. This crate converts 33023448000 to 1y 17d 5h 10m 48s

#![no_std]
extern crate alloc;

pub use pretty::splitter::Millisecond;
pub use pretty::{MillisecondFormatter, MillisecondOption};

pub use relative::parser::RelativePart;

pub mod prelude;
pub mod pretty;
pub mod relative;
pub mod weekday;

#[cfg(test)]
mod tests {

    #[test]
    fn should_work_always_with_backward_compatiblity() {
        use crate::prelude::*;

        let dur = core::time::Duration::from_millis(33_023_448_000);
        assert_eq!("1y 17d 5h 10m 48s", dur.pretty());
        assert_eq!(
            "1 year 17 days 5 hours 10 minutes 48 seconds",
            dur.pretty_with(MillisecondOption::long())
        );
        assert_eq!("about a year ago", dur.relative());

        // the previous solution still works
        let ms = Millisecond::from_millis(33_023_448_000);
        assert_eq!("1y 17d 5h 10m 48s", ms.pretty());
    }
}
