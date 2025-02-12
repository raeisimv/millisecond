use core::{
    fmt::{Display, Formatter},
    ops::Deref,
    time::Duration,
};

use alloc::string::String;

use crate::MillisecondFormatter;

/// The input value, specified in milliseconds, is parsed and decomposed into constituent
/// components such as years, days, and seconds. These components can subsequently be utilized
/// for various applications, including the formation of a human-readable string or
/// integration into your specific calculations.
///
/// ## Example
/// ```rust
/// use crate::millisecond::*;
///
/// let ms = Millisecond::from_millis(33023448000);
///
/// print!("short: {ms}");
/// // short: 1y 17d 5h 10m 48s
///
/// print!("short: {}", ms.to_short_string());
/// // short: 1y 17d 5h 10m 48s
///
/// print!("long: {}", ms.to_long_string());
/// // long: 1 year 17 days 5 hours 10 minutes 48 seconds
/// ```
///
/// ## Deref and From traits
/// This struct implements `From` and `Deref` from/to `core::time::Duration`.
///
/// ```rust
/// use crate::millisecond::*;
/// use core::time::Duration;
///
/// // convert into:
/// let ms: Millisecond = Duration::from_millis(0).into();
///
/// // calling `Duration` functions is available:
/// assert_eq!(ms.as_millis(), 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Millisecond {
    dur: Duration,
}

impl From<Duration> for Millisecond {
    fn from(value: Duration) -> Self {
        Self { dur: value }
    }
}

impl Deref for Millisecond {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.dur
    }
}
unsafe impl Sync for Millisecond {}

impl Millisecond {
    /// Creates a Millisecond instance using the provided nanoseconds.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_nanos(1_800);
    ///
    /// assert_eq!(ms.to_string(), "1µs 800ns")
    /// ```
    pub fn from_nanos(nanos: u64) -> Self {
        Duration::from_nanos(nanos).into()
    }

    /// Creates a Millisecond instance using the provided microseconds.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_micros(1_800);
    ///
    /// assert_eq!(ms.to_string(), "1ms 800µs")
    /// ```
    pub fn from_micros(micros: u64) -> Self {
        Duration::from_micros(micros).into()
    }

    /// Creates a Millisecond instance using the provided milliseconds.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_millis(1_800);
    ///
    /// assert_eq!(ms.to_string(), "1s 800ms")
    /// ```
    pub fn from_millis(millis: u64) -> Self {
        Duration::from_millis(millis).into()
    }

    /// Creates a Millisecond instance using the provided seconds.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_secs(61);
    ///
    /// assert_eq!(ms.to_string(), "1m 1s")
    /// ```
    pub fn from_secs(secs: u64) -> Self {
        Duration::from_secs(secs).into()
    }

    /// Creates a Millisecond instance using the provided minutes.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_minutes(61);
    ///
    /// assert_eq!(ms.to_string(), "1h 1m")
    /// ```
    pub fn from_minutes(minutes: u32) -> Self {
        Duration::from_secs(minutes as u64 * 60).into()
    }

    /// Creates a Millisecond instance using the provided hours.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_hours(25);
    ///
    /// assert_eq!(ms.to_string(), "1d 1h")
    /// ```
    pub fn from_hours(hours: u32) -> Self {
        Self::from_secs(hours as u64 * 60 * 60)
    }

    /// Creates a Millisecond instance using the provided days.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_days(366);
    ///
    /// assert_eq!(ms.to_string(), "1y 1d")
    /// ```
    pub fn from_days(days: u32) -> Self {
        Self::from_hours(days * 24)
    }

    /// Creates a Millisecond instance using the provided years.
    /// ### Example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_years(1);
    ///
    /// assert_eq!(ms.to_string(), "1y")
    /// ```
    pub fn from_years(years: u32) -> Self {
        Self::from_days(years * 365)
    }
}

impl MillisecondFormatter for Millisecond {
    type Output = String;

    fn to_string_with(&self, opt: &crate::MillisecondOption) -> Self::Output {
        self.dur.to_string_with(opt)
    }
}

impl Display for Millisecond {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_short_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_from_millis_basic() {
        let x = Millisecond::from_millis(10_123);
        assert_eq!(x.to_short_string(), "10s 123ms");
    }
    #[test]
    fn should_split_from_millis() {
        let cases = [
            (0, ""),
            (1, "1ms"),
            (999, "999ms"),
            (1000, "1s"),
            (1000 + 400, "1s 400ms"),
            ((1000 * 2) + 400, "2s 400ms"),
            (1000 * 55, "55s"),
            (1000 * 67, "1m 7s"),
            (1000 * 60 * 5, "5m"),
            (1000 * 60 * 67, "1h 7m"),
            (1000 * 60 * 60 * 12, "12h"),
            (1000 * 60 * 60 * 40, "1d 16h"),
            (1000 * 60 * 60 * 999, "41d 15h"),
            (1000 * 60 * 60 * 24 * 465, "1y 100d"),
            (1000 * 60 * 67 * 24 * 465, "1y 154d 6h"),
            (119_999, "1m 59s 999ms"),
            (120_000, "2m"),
            (9007199254740991, "285616y 151d 8h 59m 991ms"), // "285616y 151d 8h 59m 0.9s"
            (u64::MAX, "584942417y 129d 14h 25m 51s 615ms"),
        ];
        for (k, v) in cases {
            assert_eq!(
                Millisecond::from_millis(k).to_short_string(),
                v,
                "from_millis ({k})"
            );

            if let Some(x) = k.checked_mul(1_000) {
                assert_eq!(
                    Millisecond::from_micros(x).to_short_string(),
                    v,
                    "from_micros ({k})"
                );
            }

            if let Some(x) = k.checked_mul(1_000_000) {
                assert_eq!(
                    Millisecond::from_nanos(x).to_short_string(),
                    v,
                    "from_nanos ({k})"
                );
            }
        }
    }
    #[test]
    fn should_split_from_micros() {
        let x = Millisecond::from_micros(1).to_short_string();
        assert_eq!(x, "1µs");
        let x = Millisecond::from_micros(1_800).to_short_string();
        assert_eq!(x, "1ms 800µs");
    }
    #[test]
    fn should_split_from_nanos() {
        let x = Millisecond::from_nanos(1).to_short_string();
        assert_eq!(x, "1ns");
        let x = Millisecond::from_nanos(1_800).to_short_string();
        assert_eq!(x, "1µs 800ns");
    }
    #[test]
    fn should_split_from_secs() {
        let x = Millisecond::from_secs(1).to_short_string();
        assert_eq!(x, "1s");
    }
    #[test]
    fn should_split_from_minutes() {
        let x = Millisecond::from_minutes(1).to_short_string();
        assert_eq!(x, "1m");
        let x = Millisecond::from_minutes(61).to_short_string();
        assert_eq!(x, "1h 1m");
    }
    #[test]
    fn should_split_from_hours() {
        let x = Millisecond::from_hours(1).to_short_string();
        assert_eq!(x, "1h");
        let x = Millisecond::from_hours(25).to_short_string();
        assert_eq!(x, "1d 1h");
    }
    #[test]
    fn should_split_from_days() {
        let x = Millisecond::from_days(1).to_short_string();
        assert_eq!(x, "1d");
        let x = Millisecond::from_days(366).to_short_string();
        assert_eq!(x, "1y 1d");
    }
    #[test]
    fn should_split_from_years() {
        let x = Millisecond::from_years(1).to_short_string();
        assert_eq!(x, "1y");
    }
}
