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
    /// Determines the output format for the duration string.
    pub format: OutputFormat,

    /// When activated, displays time durations in days rather than converting them into years.
    pub days_instead_of_years: bool,

    /// When activated, displays the most dominant part only (the most left part) and produces a compact string.
    pub dominant_only: bool,

    /// When activated, shows and formats microseconds and nanoseconds.
    pub format_sub_milliseconds: bool,

    /// Determines how seconds and milliseconds should be formatted.
    /// Default is `Combine`, which combines seconds and milliseconds into a single float number with precision of 1.
    pub seconds: SecondsOptions,

    /// Determines the maximum number of units to display in the formatted string (from years towards nanoseconds).
    /// Default is `None`, which means all units will be displayed.
    /// This flag takes precedence over the `dominant_only` setting.
    pub unit_count: Option<usize>,

    /// Determines the separator used between different parts of the formatted string.
    /// Default is `Space` allowing other options to override it, e.g. `format: Colon` would replace it with a colon.
    pub separator: Separator,
}

impl MillisecondOption {
    /// Creates Options for showing a long and verbose string
    pub fn long() -> Self {
        Self {
            format: OutputFormat::Long,
            ..Default::default()
        }
    }

    /// Creates options for showing a colon-separated string
    pub fn colon() -> Self {
        Self {
            format: OutputFormat::Colon,
            seconds: SecondsOptions::Combine,
            format_sub_milliseconds: false,
            dominant_only: false,
            ..Default::default()
        }
    }

    /// Creates options to format sub-milliseconds
    pub fn sub_milliseconds() -> Self {
        Self {
            format_sub_milliseconds: true,
            ..Default::default()
        }
    }

    pub fn get_separator(&self) -> &str {
        match self.separator {
            Separator::Default => match self.format {
                OutputFormat::Colon => ":",
                _ => self.separator.as_ref(),
            },
            _ => self.separator.as_ref(),
        }
    }

    pub fn get_unit_count(&self) -> usize {
        let dominant = if self.dominant_only { 1 } else { 8 };
        self.unit_count.unwrap_or(dominant).clamp(1, 8)
    }

    #[cfg(test)]
    pub(crate) fn backward_compatible() -> Self {
        Self {
            format_sub_milliseconds: true,
            seconds: SecondsOptions::Separate,
            ..Self::default()
        }
    }
}

/// Options for formatting seconds and milliseconds; either combined or separated.
#[derive(Debug, Copy, Clone, Default)]
pub enum SecondsOptions {
    /// Separates seconds and milliseconds into two single digits.
    /// Example: 1s 2ms
    #[default]
    Separate,

    /// Combines seconds and milliseconds into a single float value.
    /// Example: 1.2s
    Combine,

    /// Combines seconds and milliseconds into a single float value with custom settings.
    /// Example: 1.23s or 01.230s
    CombineWith {
        /// Determines the number of digits to show for the milliseconds part.
        /// The default is 1, and the maximum is 3.
        /// Other values are rounded to the specified range.
        precision: u8,

        /// Determines whether milliseconds should be displayed with a fixed width.
        /// If true, seconds are always displayed with a fixed width of 2 digits,
        /// and milliseconds with a fixed width based on the `precision` option.
        fixed_width: bool,
    },

    /// Hides seconds and milliseconds
    Hide,
}
impl SecondsOptions {
    /// Returns the precision for the milliseconds part considering the fixed width option.
    pub fn precision(&self) -> u8 {
        let p = match self {
            Self::CombineWith { precision, .. } => (*precision).clamp(1, 3),
            _ => 1,
        };
        if self.is_fixed_width() { p.min(3) } else { p }
    }

    /// Returns whether the seconds and milliseconds parts should be displayed with a fixed width.
    pub fn is_fixed_width(&self) -> bool {
        match self {
            Self::CombineWith { fixed_width, .. } => *fixed_width,
            _ => false,
        }
    }
}

/// Determines the output format for the duration.
/// It can be either short, long, or colon-separated.
/// Short: `1h 2m 3s`,
/// Long: `1 hour, 2 minutes, 3 seconds`,
/// Colon: `1:02:03`
#[derive(Debug, Clone, Copy, Default)]
pub enum OutputFormat {
    /// Uses short labels for units with space separation.
    #[default]
    Short,

    /// Uses long labels for units with space separation.
    Long,

    /// Uses no labels for unit with colon (:) separation. Always shows time in at least minutes: 1s → 0:01.
    /// Useful when you want to display time without the time units, similar to a digital watch.
    Colon,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Separator {
    /// A space, allowing other options overwriting it.
    #[default]
    Default,
    /// A space
    Space,
    /// A colon (:)
    SingleColon,
    /// A comma (,)
    Comma,
    /// A dash (-)
    Dash,
    /// A slash (/)
    Slash,
    /// A pipe (|)
    Pipe,
    /// An underscore (_)
    Underscore,
}
impl AsRef<str> for Separator {
    fn as_ref(&self) -> &str {
        match self {
            Self::Default => " ",
            Self::Space => " ",
            Self::SingleColon => ":",
            Self::Comma => ",",
            Self::Dash => "-",
            Self::Slash => "/",
            Self::Pipe => "|",
            Self::Underscore => "_",
        }
    }
}
