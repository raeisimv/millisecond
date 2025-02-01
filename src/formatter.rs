use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::fmt::{Display, Formatter};

use crate::Millisecond;
pub trait MillisecondFormatter {
    fn to_string_with(&self, opt: MillisecondFormatterOptions) -> String;

    fn to_short_string(&self) -> String {
        self.to_string_with(MillisecondFormatterOptions::default())
    }

    fn to_long_string(&self) -> String {
        self.to_string_with(MillisecondFormatterOptions {
            long: true,
            ..MillisecondFormatterOptions::default()
        })
    }
}

/// Millisecond Formatting Options
#[derive(Default, Clone)]
pub struct MillisecondFormatterOptions {
    /// Uses full-length unit names: 1y 2d 3h -> 1 year 2 days 3 seconds
    pub long: bool,

    /// Converts years to days: 1y 2d 3h -> (1 * 365 + 2) -> 367d 3h. ```Default: false```
    pub days_instead_of_years: bool,

    /// Converts years and days to hours: 1y 2d 3h 4ms -> ((1 * 365 + 2) * 24 + 3) -> 8811h 4ms. ```Default: false```
    pub hours_instead_of_days: bool,
}

impl MillisecondFormatter for Millisecond {
    fn to_string_with(&self, opt: MillisecondFormatterOptions) -> String {
        let mut res: Vec<MillisecondPart> = Vec::with_capacity(9);

        if opt.hours_instead_of_days {
            let v = self.years * 365 + self.days as u64 * 24 + self.hours as u64;
            if v > 0 {
                res.push(MillisecondPart::Hours(v));
            }
        } else if opt.days_instead_of_years {
            let v = self.years * 365 + self.days as u64 * 24;
            if v > 0 {
                res.push(MillisecondPart::Days(v));
            }
            if self.hours > 0 {
                res.push(MillisecondPart::Hours(self.hours as _));
            }
        } else {
            if self.years > 0 {
                res.push(MillisecondPart::Years(self.years as _));
            }
            if self.days > 0 {
                res.push(MillisecondPart::Days(self.days as _));
            }
            if self.hours > 0 {
                res.push(MillisecondPart::Hours(self.hours as _));
            }
        }

        res.iter()
            .map(|x| {
                if opt.long {
                    x.to_long_string()
                } else {
                    x.to_short_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Debug)]
pub enum MillisecondPart {
    Years(u64),
    Days(u64),
    Hours(u64),
    Minutes(u8),
    Seconds(u8),
    SecsAndMillis(u8, u16),
    Millis(u16),
    Micros(u16),
    Nanos(u16),
}

impl MillisecondPart {
    pub fn to_short_string(&self) -> String {
        match self {
            MillisecondPart::Years(x) => format!("{x}y"),
            MillisecondPart::Days(x) => format!("{x}d"),
            MillisecondPart::Hours(x) => format!("{x}h"),
            MillisecondPart::Minutes(x) => format!("{x}m"),
            MillisecondPart::Seconds(x) => format!("{x}s"),
            MillisecondPart::Millis(x) => format!("{x}ms"),
            MillisecondPart::SecsAndMillis(x, y) => format!("{x}.{y}s"),
            MillisecondPart::Micros(x) => format!("{x}µs"),
            MillisecondPart::Nanos(x) => format!("{x}ns"),
        }
    }
    pub fn to_long_string(&self) -> String {
        match self {
            MillisecondPart::Years(x) => with_pluralization(x, "year", 1),
            MillisecondPart::Days(x) => with_pluralization(x, "day", 1),
            MillisecondPart::Hours(x) => with_pluralization(x, "hour", 1),
            MillisecondPart::Minutes(x) => with_pluralization(x, "minute", 1),
            MillisecondPart::Seconds(x) => with_pluralization(x, "second", 1),
            MillisecondPart::Millis(x) => with_pluralization(x, "millisecond", 1),
            MillisecondPart::SecsAndMillis(x, y) => {
                format!("{x}.{y} seconds")
            }
            MillisecondPart::Micros(x) => with_pluralization(x, "microsecond", 1),
            MillisecondPart::Nanos(x) => with_pluralization(x, "nanosecond", 1),
        }
    }
    pub fn from_millisecond(ms: &Millisecond) -> Vec<MillisecondPart> {
        Self::from_millisecond_with_option(ms, true)
    }
    pub fn from_millisecond_with_option(
        ms: &Millisecond,
        merge_secs_and_millis: bool,
    ) -> Vec<MillisecondPart> {
        let mut v = vec![];
        if ms.years > 0 {
            v.push(MillisecondPart::Years(ms.years));
        }
        if ms.days > 0 {
            v.push(MillisecondPart::Days(ms.days as _));
        }
        if ms.hours > 0 {
            v.push(MillisecondPart::Hours(ms.hours as _));
        }
        if ms.minutes > 0 {
            v.push(MillisecondPart::Minutes(ms.minutes));
        }

        if ms.seconds > 0 {
            if merge_secs_and_millis && ms.millis > 0 {
                v.push(MillisecondPart::SecsAndMillis(ms.seconds, ms.millis));
            } else {
                v.push(MillisecondPart::Seconds(ms.seconds));
            }
        }
        if !merge_secs_and_millis && ms.millis > 0 {
            v.push(MillisecondPart::Millis(ms.millis));
        }

        if ms.micros > 0 {
            v.push(MillisecondPart::Micros(ms.micros));
        }
        if ms.nanos > 0 {
            v.push(MillisecondPart::Nanos(ms.nanos));
        }

        v
    }
}

impl Display for MillisecondPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_short_string())
    }
}

unsafe impl Sync for MillisecondPart {}

fn with_pluralization<T: Eq + Display>(val: &T, text: &str, single_val: T) -> String {
    if *val == single_val {
        format!("{val} {text}")
    } else {
        format!("{val} {text}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_string() {
        let act = Millisecond::from_years(1).to_string_with(MillisecondFormatterOptions::default());
        assert_eq!(act, "1y");
    }
}
