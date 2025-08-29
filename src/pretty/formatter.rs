use alloc::string::String;
use core::time::Duration;

use crate::MillisecondOption;

/// Protocol for being a Millisecond Formatter.
/// The protocol is implemented for `core::time::Duration` and `Millisecond` structs.
pub trait MillisecondFormatter {
    type Output;

    /// Returns human-readable and pretty string of the given value/struct with custom options
    fn pretty_with(&self, opt: MillisecondOption) -> Self::Output;

    /// Returns human-readable and pretty string of the given value/struct with default options
    fn pretty(&self) -> Self::Output {
        self.pretty_with(MillisecondOption::default())
    }

    /// Returns human-readable text in a short string.
    /// ### DEPRECATED
    /// Use the `pretty` function instead.
    #[deprecated(since = "0.4.0", note = "use the `pretty` instead")]
    fn to_short_string(&self) -> Self::Output {
        self.pretty()
    }

    /// Returns human-readable text in a long and verbose string.
    /// ### DEPRECATED
    /// Use the `pretty_with` function instead.
    #[deprecated(since = "0.4.0", note = "use the `pretty_with` function instead")]
    fn to_long_string(&self) -> Self::Output {
        self.pretty_with(MillisecondOption::long())
    }
}

impl MillisecondFormatter for Duration {
    type Output = String;

    fn pretty_with(&self, opt: MillisecondOption) -> Self::Output {
        let parts = super::parser::parse_duration(self, &opt);
        super::parser::ms_parts_to_string(&parts, &opt)
    }
}
