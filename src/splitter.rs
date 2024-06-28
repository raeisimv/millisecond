use core::fmt::{Display, Formatter};

use crate::formatter::MillisecondPart;

/// The input value (milliseconds) will be parsed and separated into its parts,
/// including years, days, seconds, etc.
/// These part then can be used for different purposes such as formatting a human-readable string
/// or participate in desired calculations.
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn from_millis(val: u128) -> Self {
        let total_seconds = (val / 1000) as u64;
        let total_minutes = total_seconds / 60;
        let total_hours = total_minutes / 60;
        let total_days = total_hours / 24;

        Self {
            years: total_days / 365,
            days: (total_days % 365) as u16,
            hours: (total_hours % 24) as u8,
            minutes: (total_minutes % 60) as u8,
            seconds: (total_seconds % 60) as u8,
            millis: (val % 1000) as u16,
            micros: 0,
            nanos: 0,
        }
    }
    pub fn from_nanos(val: u128) -> Self {
        let total_micros = val / 1000;
        let total_millis = total_micros / 1000;
        let total_seconds = (total_millis / 1000) as u64;
        let total_minutes = total_seconds / 60;
        let total_hours = total_minutes / 60;
        let total_days = total_hours / 24;

        Self {
            years: total_days / 365,
            days: (total_days % 365) as u16,
            hours: (total_hours % 24) as u8,
            minutes: (total_minutes % 60) as u8,
            seconds: (total_seconds % 60) as u8,
            millis: (total_millis % 1000) as u16,
            micros: (total_micros % 1000) as u16,
            nanos: (val % 1000) as u16,
        }
    }
    pub fn from_micros(val: u128) -> Self {
        Self::from_nanos(val * 1000)
    }
    pub fn from_secs(val: u64) -> Self {
        Self::from_millis(val as u128 * 1000)
    }
    pub fn from_minutes(val: u64) -> Self {
        Self::from_secs(val * 60)
    }
    pub fn from_hours(val: u64) -> Self {
        Self::from_minutes(val * 60)
    }
    pub fn from_days(val: u64) -> Self {
        Self::from_hours(val * 24)
    }
    pub fn from_years(val: u64) -> Self {
        Self::from_days(val * 365)
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