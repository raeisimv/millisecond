use alloc::string::String;
use core::time::Duration;

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

/// The options struct serves as a configuration mechanism for both parsing input and producing
/// the final formatted output. It allows you to customize the behavior and settings used during
/// these processes to tailor the results according to your specific requirements.
#[derive(Debug, Copy, Clone, Default)]
pub struct MillisecondOption {
    /// When enabled, uses full and descriptive labels for time units, such as `years` instead of abbreviated forms like `y`.
    pub long: bool,

    /// When activated, displays time durations in days rather than converting them into years.
    pub days_instead_of_years: bool,

    /// When activated, displays the most dominant part only (the most left part).
    pub dominant_only: bool,

    /// When activated, shows and formats microseconds and nanoseconds.
    pub format_sub_milliseconds: bool,

    /// When activated, separates the seconds from the milliseconds into two single digits.
    pub separate_milliseconds: bool,
}

impl MillisecondOption {
    /// Creates Options for showing a long and verbose string
    pub fn long() -> Self {
        Self {
            long: true,
            ..Default::default()
        }
    }
    pub fn sub_milliseconds() -> Self {
        Self {
            format_sub_milliseconds: true,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        Self {
            format_sub_milliseconds: true,
            separate_milliseconds: true,
            ..Self::default()
        }
    }
}
