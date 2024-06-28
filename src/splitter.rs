use core::fmt::{Display, Formatter};

use crate::formatter::MillisecondPart;

/// The input value, specified in milliseconds, is parsed and decomposed into constituent
/// components such as years, days, and seconds. These components can subsequently be utilized
/// for various applications, including the formation of a human-readable string or 
/// integration into your specific calculations.
/// ## Example
/// ```rust
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
///  years: 1,
///  days: 17,
///  hours: 5,
///  minutes: 10,
///  seconds: 48,
///  millis: 0,
///  micros: 0,
///  nanos: 0,
/// });
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Millisecond {
    pub years: u64,
    pub days: u16,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub millis: u16,
    pub micros: u16,
    pub nanos: u16,
}

unsafe impl Sync for Millisecond {}

impl Millisecond {
    /// Creates a Millisecond instance using the provided nanoseconds.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_nanos(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 0,
    ///   hours: 0,
    ///   minutes: 0,
    ///   seconds: 0,
    ///   millis: 0,
    ///   micros: 1,
    ///   nanos: 800,
    /// })
    /// ```
    pub fn from_nanos(nanos: u128) -> Self {
        Self {
            nanos: (nanos % 1000) as u16,
            ..Self::from_micros(nanos / 1000)
        }
    }

    /// Creates a Millisecond instance using the provided microseconds.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_micros(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 0,
    ///   hours: 0,
    ///   minutes: 0,
    ///   seconds: 0,
    ///   millis: 1,
    ///   micros: 800,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_micros(micros: u128) -> Self {
        Self {
            micros: (micros % 1000) as u16,
            ..Self::from_millis(micros / 1000)
        }
    }

    /// Creates a Millisecond instance using the provided milliseconds.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_millis(1_800);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 0,
    ///   hours: 0,
    ///   minutes: 0,
    ///   seconds: 1,
    ///   millis: 800,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_millis(millis: u128) -> Self {
        Self {
            millis: (millis % 1000) as u16,
            ..Self::from_secs((millis / 1000) as u64)
        }
    }


    /// Creates a Millisecond instance using the provided seconds.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_secs(61);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 0,
    ///   hours: 0,
    ///   minutes: 1,
    ///   seconds: 1,
    ///   millis: 0,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_secs(seconds: u64) -> Self {
        Self {
            seconds: (seconds % 60) as u8,
            ..Self::from_minutes(seconds / 60)
        }
    }

    /// Creates a Millisecond instance using the provided minutes.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_minutes(61);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 0,
    ///   hours: 1,
    ///   minutes: 1,
    ///   seconds: 0,
    ///   millis: 0,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            minutes: (minutes % 60) as u8,
            ..Self::from_hours(minutes / 60)
        }
    }


    /// Creates a Millisecond instance using the provided hours.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_hours(25);
    /// assert_eq!(ms, Millisecond {
    ///   years: 0,
    ///   days: 1,
    ///   hours: 1,
    ///   minutes: 0,
    ///   seconds: 0,
    ///   millis: 0,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_hours(hours: u64) -> Self {
        Self {
            hours: (hours % 24) as u8,
            ..Self::from_days(hours / 24)
        }
    }

    /// Creates a Millisecond instance using the provided days.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_hours(366);
    /// assert_eq!(ms, Millisecond {
    ///   years: 1,
    ///   days: 1,
    ///   hours: 0,
    ///   minutes: 0,
    ///   seconds: 0,
    ///   millis: 0,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_days(days: u64) -> Self {
        Self {
            days: (days % 365) as u16,
            ..Self::from_years(days / 365)
        }
    }

    /// Creates a Millisecond instance using the provided years.
    /// ### example
    /// ```rust
    /// use millisecond::Millisecond;
    /// let ms = Millisecond::from_years(1);
    /// assert_eq!(ms, Millisecond {
    ///   years: 1,
    ///   days: 0,
    ///   hours: 0,
    ///   minutes: 0,
    ///   seconds: 0,
    ///   millis: 0,
    ///   micros: 0,
    ///   nanos: 0,
    /// })
    /// ```
    pub fn from_years(years: u64) -> Self {
        Self {
            years,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        }
    }
}

impl Millisecond {
    pub fn to_short_string(&self) -> String {
        MillisecondPart::from_millisecond(self)
            .iter()
            .map(|x| x.to_short_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn to_long_string(&self) -> String {
        MillisecondPart::from_millisecond(self)
            .iter()
            .map(|x| x.to_long_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
impl Display for Millisecond {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_short_string())
    }
}
#[cfg(test)]
mod tests {
    use Millisecond;

    use super::*;

    #[test]
    fn should_split_from_millis_basic() {
        let x = Millisecond::from_millis(10_123);
        assert_eq!(x.years, 0);
        assert_eq!(x.days, 0);
        assert_eq!(x.hours, 0);
        assert_eq!(x.minutes, 0);
        assert_eq!(x.seconds, 10);
        assert_eq!(x.millis, 123);
    }
    #[test]
    fn should_split_from_millis() {
        let cases = [
            (0, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                millis: 1,
                micros: 0,
                nanos: 0,
            }),
            (999, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                millis: 999,
                micros: 0,
                nanos: 0,
            }),
            (1000, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 1,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 + 400, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 1,
                millis: 400,
                micros: 0,
                nanos: 0,
            }),
            ((1000 * 2) + 400, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 2,
                millis: 400,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 55, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 55,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 67, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 7,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 5, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 5,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 67, Millisecond {
                years: 0,
                days: 0,
                hours: 1,
                minutes: 7,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 60 * 12, Millisecond {
                years: 0,
                days: 0,
                hours: 12,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 60 * 40, Millisecond {
                years: 0,
                days: 1,
                hours: 16,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 60 * 999, Millisecond {
                years: 0,
                days: 41,
                hours: 15,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 60 * 24 * 465, Millisecond {
                years: 1,
                days: 100,
                hours: 0,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (1000 * 60 * 67 * 24 * 465, Millisecond {
                years: 1,
                days: 154,
                hours: 6,
                minutes: 0,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (119_999, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 59,
                millis: 999,
                micros: 0,
                nanos: 0,
            }),
            (120_000, Millisecond {
                years: 0,
                days: 0,
                hours: 0,
                minutes: 2,
                seconds: 0,
                millis: 0,
                micros: 0,
                nanos: 0,
            }),
            (9007199254740991, Millisecond {
                years: 285616,
                days: 151,
                hours: 8,
                minutes: 59,
                seconds: 0,
                millis: 991,
                micros: 0,
                nanos: 0,
            }), // "285616y 151d 8h 59m 0.9s"
            (u64::MAX as u128, Millisecond {
                years: 584942417,
                days: 129,
                hours: 14,
                minutes: 25,
                seconds: 51,
                millis: 615,
                micros: 0,
                nanos: 0,
            }),
            (u128::MAX, Millisecond {
                years: 360324529090,
                days: 264,
                hours: 9,
                minutes: 29,
                seconds: 55,
                millis: 455,
                micros: 0,
                nanos: 0,
            }),
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
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 1,
            nanos: 0,
        });
        let x = Millisecond::from_micros(1_800);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 1,
            micros: 800,
            nanos: 0,
        });
    }
    #[test]
    fn should_split_from_nanos() {
        let x = Millisecond::from_nanos(1);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 1,
        });
        let x = Millisecond::from_nanos(1_800);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 1,
            nanos: 800,
        });
    }
    #[test]
    fn should_split_from_secs() {
        let x = Millisecond::from_secs(1);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 1,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
    }
    #[test]
    fn should_split_from_minutes() {
        let x = Millisecond::from_minutes(1);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 0,
            minutes: 1,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
        let x = Millisecond::from_minutes(61);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 1,
            minutes: 1,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
    }
    #[test]
    fn should_split_from_hours() {
        let x = Millisecond::from_hours(1);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 0,
            hours: 1,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
        let x = Millisecond::from_hours(25);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 1,
            hours: 1,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
    }
    #[test]
    fn should_split_from_days() {
        let x = Millisecond::from_days(1);
        assert_eq!(x, Millisecond {
            years: 0,
            days: 1,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
        let x = Millisecond::from_days(366);
        assert_eq!(x, Millisecond {
            years: 1,
            days: 1,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
    }
    #[test]
    fn should_split_from_years() {
        let x = Millisecond::from_years(1);
        assert_eq!(x, Millisecond {
            years: 1,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            millis: 0,
            micros: 0,
            nanos: 0,
        });
    }
}