use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

use crate::formatter::MillisecondPart;

/// The input value, specified in milliseconds, is parsed and decomposed into constituent
/// components such as years, days, and seconds. These components can subsequently be utilized
/// for various applications, including the formation of a human-readable string or
/// integration into your specific calculations.
/// ## Example
/// ```rust
/// use millisecond::MillisecondPart;
/// use crate::millisecond::Millisecond;
///
/// let ms = Millisecond::from_millis(33023448000);
///
/// print!("short: {ms}");
/// // short: 1y 17d 5h 10m 48s
///
/// print!("long: {}", ms.to_long_string());
/// // long: 1 year 17 days 5 hours 10 minutes 48 seconds
///
/// assert_eq!(ms, Millisecond {
///  parts: vec![
///     MillisecondPart::Years(1),
///     MillisecondPart::Days(17),
///     MillisecondPart::Hours(5),
///     MillisecondPart::Minutes(10),
///     MillisecondPart::Seconds(48)],
/// });
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Millisecond {
    pub parts: Vec<MillisecondPart>,
}

unsafe impl Sync for Millisecond {}

impl Millisecond {
    /// Creates a Millisecond instance using the provided nanoseconds.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_nanos(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Micros(1), MillisecondPart::Nanos(800)],
    /// })
    /// ```
    pub fn from_nanos(nanos: u128) -> Self {
        let micros = nanos / 1000;
        let nanos = (nanos % 1000) as u16;
        let mut nanos = if nanos > 0 {
            vec![MillisecondPart::Nanos(nanos)]
        } else {
            vec![]
        };
        let mut ms = Self::from_micros(micros);
        ms.parts.append(&mut nanos);
        ms
    }

    /// Creates a Millisecond instance using the provided microseconds.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_micros(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Millis(1), MillisecondPart::Micros(800)],
    /// })
    /// ```
    pub fn from_micros(micros: u128) -> Self {
        let millis = micros / 1000;
        let micros = (micros % 1000) as u16;
        let mut micros = if micros > 0 {
            vec![MillisecondPart::Micros(micros)]
        } else {
            vec![]
        };
        let mut ms = Self::from_millis(millis);
        ms.parts.append(&mut micros);
        ms
    }

    /// Creates a Millisecond instance using the provided milliseconds.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_millis(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Seconds(1), MillisecondPart::Millis(800)],
    /// })
    /// ```
    pub fn from_millis(millis: u128) -> Self {
        let secs = (millis / 1000) as u64;
        let millis = (millis % 1000) as u16;
        let mut millis = if millis > 0 {
            vec![MillisecondPart::Millis(millis)]
        } else {
            vec![]
        };
        let mut ms = Self::from_secs(secs);
        ms.parts.append(&mut millis);
        ms
    }

    /// Creates a Millisecond instance using the provided seconds.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_secs(61);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Minutes(1), MillisecondPart::Seconds(1)],
    /// })
    /// ```
    pub fn from_secs(seconds: u64) -> Self {
        let minutes = seconds / 60;
        let seconds = (seconds % 60) as u8;
        let mut seconds = if seconds > 0 {
            vec![MillisecondPart::Seconds(seconds)]
        } else {
            vec![]
        };
        let mut ms = Self::from_minutes(minutes);
        ms.parts.append(&mut seconds);
        ms
    }

    /// Creates a Millisecond instance using the provided minutes.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_minutes(61);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Hours(1), MillisecondPart::Minutes(1)],
    /// })
    /// ```
    pub fn from_minutes(minutes: u64) -> Self {
        let hours = minutes / 60;
        let minutes = (minutes % 60) as u8;
        let mut minutes = if minutes > 0 {
            vec![MillisecondPart::Minutes(minutes)]
        } else {
            vec![]
        };

        let mut ms = Self::from_hours(hours);
        ms.parts.append(&mut minutes);
        ms
    }

    /// Creates a Millisecond instance using the provided hours.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_hours(25);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Days(1), MillisecondPart::Hours(1)],
    /// })
    /// ```
    pub fn from_hours(hours: u64) -> Self {
        let days = hours / 24;
        let hours = (hours % 24) as u8;
        let mut hours = if hours > 0 {
            vec![MillisecondPart::Hours(hours)]
        } else {
            vec![]
        };

        let mut ms = Self::from_days(days);
        ms.parts.append(&mut hours);
        ms
    }

    /// Creates a Millisecond instance using the provided days.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_days(366);
    /// assert_eq!(ms, Millisecond {
    ///  parts: vec![MillisecondPart::Years(1), MillisecondPart::Days(1)]
    /// })
    /// ```
    pub fn from_days(days: u64) -> Self {
        let years = days / 365;
        let days = (days % 365) as u16;
        let mut days = if days > 0 {
            vec![MillisecondPart::Days(days)]
        } else {
            vec![]
        };
        let mut ms = Self::from_years(years);
        ms.parts.append(&mut days);
        ms
    }

    /// Creates a Millisecond instance using the provided years.
    /// ### example
    /// ```rust
    /// use millisecond::{Millisecond, MillisecondPart};
    /// let ms = Millisecond::from_years(1);
    /// assert_eq!(ms, Millisecond {
    ///   parts: vec![MillisecondPart::Years(1)],
    /// })
    /// ```
    pub fn from_years(years: u64) -> Self {
        Self {
            parts: if years > 0 {
                vec![MillisecondPart::Years(years)]
            } else {
                vec![]
            },
        }
    }
}

impl Millisecond {
    pub fn to_short_string(&self) -> String {
        self.parts
            .iter()
            .map(|x| x.to_short_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn to_long_string(&self) -> String {
        self.parts
            .iter()
            .map(|x| x.to_long_string())
            .collect::<Vec<String>>()
            .join(" ")
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
        let ms = Millisecond::from_millis(10_123);
        assert_eq!(
            ms,
            Millisecond {
                parts: vec![MillisecondPart::Seconds(10), MillisecondPart::Millis(123)]
            }
        );
    }
    #[test]
    fn should_split_from_millis() {
        let cases = [
            (0, Millisecond { parts: vec![] }),
            (
                1,
                Millisecond {
                    parts: vec![MillisecondPart::Millis(1)],
                },
            ),
            (
                999,
                Millisecond {
                    parts: vec![MillisecondPart::Millis(999)],
                },
            ),
            (
                1000,
                Millisecond {
                    parts: vec![MillisecondPart::Seconds(1)],
                },
            ),
            (
                1000 + 400,
                Millisecond {
                    parts: vec![MillisecondPart::Seconds(1), MillisecondPart::Millis(400)],
                },
            ),
            (
                (1000 * 2) + 400,
                Millisecond {
                    parts: vec![MillisecondPart::Seconds(2), MillisecondPart::Millis(400)],
                },
            ),
            (
                1000 * 55,
                Millisecond {
                    parts: vec![MillisecondPart::Seconds(55)],
                },
            ),
            (
                1000 * 67,
                Millisecond {
                    parts: vec![MillisecondPart::Minutes(1), MillisecondPart::Seconds(7)],
                },
            ),
            (
                1000 * 60 * 5,
                Millisecond {
                    parts: vec![MillisecondPart::Minutes(5)],
                },
            ),
            (
                1000 * 60 * 67,
                Millisecond {
                    parts: vec![MillisecondPart::Hours(1), MillisecondPart::Minutes(7)],
                },
            ),
            (
                1000 * 60 * 60 * 12,
                Millisecond {
                    parts: vec![MillisecondPart::Hours(12)],
                },
            ),
            (
                1000 * 60 * 60 * 40,
                Millisecond {
                    parts: vec![MillisecondPart::Days(1), MillisecondPart::Hours(16)],
                },
            ),
            (
                1000 * 60 * 60 * 999,
                Millisecond {
                    parts: vec![MillisecondPart::Days(41), MillisecondPart::Hours(15)],
                },
            ),
            (
                1000 * 60 * 60 * 24 * 465,
                Millisecond {
                    parts: vec![MillisecondPart::Years(1), MillisecondPart::Days(100)],
                },
            ),
            (
                1000 * 60 * 67 * 24 * 465,
                Millisecond {
                    parts: vec![
                        MillisecondPart::Years(1),
                        MillisecondPart::Days(154),
                        MillisecondPart::Hours(6),
                    ],
                },
            ),
            (
                119_999,
                Millisecond {
                    parts: vec![
                        MillisecondPart::Minutes(1),
                        MillisecondPart::Seconds(59),
                        MillisecondPart::Millis(999),
                    ],
                },
            ),
            (
                120_000,
                Millisecond {
                    parts: vec![MillisecondPart::Minutes(2)],
                },
            ),
            (
                9007199254740991,
                Millisecond {
                    parts: vec![
                        MillisecondPart::Years(285616),
                        MillisecondPart::Days(151),
                        MillisecondPart::Hours(8),
                        MillisecondPart::Minutes(59),
                        MillisecondPart::Millis(991),
                    ],
                },
            ), // "285616y 151d 8h 59m 0.9s"
            (
                u64::MAX as u128,
                Millisecond {
                    parts: vec![
                        MillisecondPart::Years(584942417),
                        MillisecondPart::Days(129),
                        MillisecondPart::Hours(14),
                        MillisecondPart::Minutes(25),
                        MillisecondPart::Seconds(51),
                        MillisecondPart::Millis(615),
                    ],
                },
            ),
            (
                u128::MAX,
                Millisecond {
                    parts: vec![
                        MillisecondPart::Years(360324529090),
                        MillisecondPart::Days(264),
                        MillisecondPart::Hours(9),
                        MillisecondPart::Minutes(29),
                        MillisecondPart::Seconds(55),
                        MillisecondPart::Millis(455),
                    ],
                },
            ),
        ];
        for (k, v) in cases {
            assert_eq!(Millisecond::from_millis(k), v, "from_millis ({k})");

            if let Some(x) = k.checked_mul(1_000) {
                assert_eq!(Millisecond::from_micros(x), v, "from_micros ({k})");
            }

            if let Some(x) = k.checked_mul(1_000_000) {
                assert_eq!(Millisecond::from_nanos(x), v, "from_nanos ({k})");
            }
        }
    }
    #[test]
    fn should_split_from_micros() {
        let x = Millisecond::from_micros(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Micros(1)]
            }
        );
        let x = Millisecond::from_micros(1_800);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Millis(1), MillisecondPart::Micros(800)],
            }
        );
    }
    #[test]
    fn should_split_from_nanos() {
        let x = Millisecond::from_nanos(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Nanos(1)]
            }
        );
        let x = Millisecond::from_nanos(1_800);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Micros(1), MillisecondPart::Nanos(800)]
            }
        );
    }
    #[test]
    fn should_split_from_secs() {
        let x = Millisecond::from_secs(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Seconds(1)]
            }
        );
    }
    #[test]
    fn should_split_from_minutes() {
        let x = Millisecond::from_minutes(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Minutes(1)]
            }
        );
        let x = Millisecond::from_minutes(61);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Hours(1), MillisecondPart::Minutes(1)]
            }
        );
    }
    #[test]
    fn should_split_from_hours() {
        let x = Millisecond::from_hours(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Hours(1)]
            }
        );
        let x = Millisecond::from_hours(25);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Days(1), MillisecondPart::Hours(1)]
            }
        );
    }
    #[test]
    fn should_split_from_days() {
        let x = Millisecond::from_days(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Days(1)]
            }
        );
        let x = Millisecond::from_days(366);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Years(1), MillisecondPart::Days(1)]
            }
        );
    }
    #[test]
    fn should_split_from_years() {
        let x = Millisecond::from_years(1);
        assert_eq!(
            x,
            Millisecond {
                parts: vec![MillisecondPart::Years(1)]
            }
        );
    }
}
