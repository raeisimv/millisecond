use alloc::{format, string::String, vec::Vec};
use core::time::Duration;

pub trait MillisecondFormatter {
    type Output;

    fn pretty_with(&self, opt: &MillisecondOption) -> Self::Output;

    fn pretty(&self) -> Self::Output {
        self.pretty_with(&MillisecondOption::default())
    }

    #[deprecated(since = "0.4.0", note = "use the `pretty` function instead")]
    fn to_short_string(&self) -> Self::Output {
        self.pretty()
    }

    #[deprecated(since = "0.4.0", note = "use the `pretty_with` function instead")]
    fn to_long_string(&self) -> Self::Output {
        self.pretty_with(&MillisecondOption::long())
    }
}

impl MillisecondFormatter for Duration {
    type Output = String;

    fn pretty_with(&self, opt: &MillisecondOption) -> Self::Output {
        let parts = parse_duration(self, opt);
        ms_parts_to_string(&parts, opt)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MillisecondOption {
    pub long: bool,
    pub days_instead_of_years: bool,
}
impl MillisecondOption {
    pub fn long() -> Self {
        Self {
            long: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MillisecondPart {
    Years(u64),
    Days(u64),
    Hours(u8),
    Minutes(u8),
    Seconds(u8),
    Millis(u16),
    Micros(u16),
    Nanos(u16),
}

pub fn parse_duration(dur: &Duration, opt: &MillisecondOption) -> [Option<MillisecondPart>; 8] {
    let mut parts = [None; 8];
    if dur.is_zero() {
        return parts;
    }

    use crate::utils::{parse_days, parse_hours, parse_mins, parse_secs, ParsedUnitValue};

    let total_nanos = dur.subsec_nanos();
    if total_nanos > 0 {
        let nanos = total_nanos % 1_000;
        if nanos > 0 {
            parts[7] = Some(MillisecondPart::Nanos(nanos as _));
        }

        let micros = dur.subsec_micros();
        if micros > 0 {
            let micros = micros % 1000;
            if micros > 0 {
                parts[6] = Some(MillisecondPart::Micros((micros % 1000) as _));
            }

            let millis = dur.subsec_millis();
            if millis > 0 {
                parts[5] = Some(MillisecondPart::Millis(millis as _));
            }
        }
    }

    let ParsedUnitValue { unit: secs, total } = parse_secs(dur.as_secs());
    if secs > 0 {
        parts[4] = Some(MillisecondPart::Seconds(secs));
    }
    if total > 0 {
        let ParsedUnitValue { unit: mins, total } = parse_mins(total);
        if mins > 0 {
            parts[3] = Some(MillisecondPart::Minutes(mins));
        }

        if total > 0 {
            let ParsedUnitValue { unit: hours, total } = parse_hours(total);
            if hours > 0 {
                parts[2] = Some(MillisecondPart::Hours(hours));
            }

            if total > 0 {
                if opt.days_instead_of_years {
                    parts[1] = Some(MillisecondPart::Days(total));
                } else {
                    let ParsedUnitValue {
                        unit: days,
                        total: years,
                    } = parse_days(total);
                    if days > 0 {
                        parts[1] = Some(MillisecondPart::Days(days as _));
                    }
                    if years > 0 {
                        parts[0] = Some(MillisecondPart::Years(years));
                    }
                }
            }
        }
    }

    parts
}

impl MillisecondPart {
    pub fn get_label(&self, long: bool) -> String {
        match self {
            MillisecondPart::Years(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} years")
                    } else {
                        format!("{x} year")
                    }
                } else {
                    format!("{x}y")
                }
            }
            MillisecondPart::Days(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} days")
                    } else {
                        format!("{x} day")
                    }
                } else {
                    format!("{x}d")
                }
            }
            MillisecondPart::Hours(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} hours")
                    } else {
                        format!("{x} hour")
                    }
                } else {
                    format!("{x}h")
                }
            }
            MillisecondPart::Minutes(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} minutes")
                    } else {
                        format!("{x} minute")
                    }
                } else {
                    format!("{x}m")
                }
            }
            MillisecondPart::Seconds(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} seconds")
                    } else {
                        format!("{x} second")
                    }
                } else {
                    format!("{x}s")
                }
            }
            MillisecondPart::Millis(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} milliseconds")
                    } else {
                        format!("{x} millisecond")
                    }
                } else {
                    format!("{x}ms")
                }
            }
            MillisecondPart::Micros(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} microseconds")
                    } else {
                        format!("{x} microsecond")
                    }
                } else {
                    format!("{x}µs")
                }
            }
            MillisecondPart::Nanos(x) => {
                if long {
                    if *x != 1 {
                        format!("{x} nanoseconds")
                    } else {
                        format!("{x} nanosecond")
                    }
                } else {
                    format!("{x}ns")
                }
            }
        }
    }
}

pub fn ms_parts_to_string(parts: &[Option<MillisecondPart>; 8], opt: &MillisecondOption) -> String {
    parts
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().get_label(opt.long))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_duration_of_ones() {
        let cases = [
            (
                Duration::new(0, 0),
                [None, None, None, None, None, None, None, None],
            ),
            (
                Duration::from_secs(1),
                [
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Seconds(1)),
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1),
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Millis(1)),
                    None,
                    None,
                ],
            ),
            (
                Duration::from_micros(1),
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Micros(1)),
                    None,
                ],
            ),
            (
                Duration::from_nanos(1),
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Nanos(1)),
                ],
            ),
        ];

        for (dur, exp) in cases.iter() {
            let parts = parse_duration(dur, &MillisecondOption::default());
            assert_eq!(parts, *exp);
        }
    }
    #[test]
    fn should_parse_duration() {
        let cases = [
            (
                Duration::from_millis(1_000),
                [
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Seconds(1)),
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1_400),
                [
                    None,
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Seconds(1)),
                    Some(MillisecondPart::Millis(400)),
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 67),
                [
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Minutes(1)),
                    Some(MillisecondPart::Seconds(7)),
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 67),
                [
                    None,
                    None,
                    Some(MillisecondPart::Hours(1)),
                    Some(MillisecondPart::Minutes(7)),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 60 * 12),
                [
                    None,
                    None,
                    Some(MillisecondPart::Hours(12)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 60 * 40),
                [
                    None,
                    Some(MillisecondPart::Days(1)),
                    Some(MillisecondPart::Hours(16)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 60 * 999),
                [
                    None,
                    Some(MillisecondPart::Days(41)),
                    Some(MillisecondPart::Hours(15)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 60 * 24 * 465),
                [
                    Some(MillisecondPart::Years(1)),
                    Some(MillisecondPart::Days(100)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(1000 * 60 * 67 * 24 * 465),
                [
                    Some(MillisecondPart::Years(1)),
                    Some(MillisecondPart::Days(154)),
                    Some(MillisecondPart::Hours(6)),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(119_999),
                [
                    None,
                    None,
                    None,
                    Some(MillisecondPart::Minutes(1)),
                    Some(MillisecondPart::Seconds(59)),
                    Some(MillisecondPart::Millis(999)),
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(9007199254740991),
                [
                    Some(MillisecondPart::Years(285616)),
                    Some(MillisecondPart::Days(151)),
                    Some(MillisecondPart::Hours(8)),
                    Some(MillisecondPart::Minutes(59)),
                    None,
                    Some(MillisecondPart::Millis(991)),
                    None,
                    None,
                ],
            ),
            (
                Duration::from_millis(u64::MAX),
                [
                    Some(MillisecondPart::Years(584942417)),
                    Some(MillisecondPart::Days(129)),
                    Some(MillisecondPart::Hours(14)),
                    Some(MillisecondPart::Minutes(25)),
                    Some(MillisecondPart::Seconds(51)),
                    Some(MillisecondPart::Millis(615)),
                    None,
                    None,
                ],
            ),
        ];

        for (dur, exp) in cases.iter() {
            let parts = parse_duration(dur, &MillisecondOption::default());
            assert_eq!(parts, *exp);
        }
    }
    #[test]
    fn should_convert_to_string() {
        let test_cases = [
            (
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                "",
                ""
            ),
            (
                [
                    Some(MillisecondPart::Years(1)),
                    Some(MillisecondPart::Days(1)),
                    Some(MillisecondPart::Hours(1)),
                    Some(MillisecondPart::Minutes(1)),
                    Some(MillisecondPart::Seconds(1)),
                    Some(MillisecondPart::Millis(1)),
                    Some(MillisecondPart::Micros(1)),
                    Some(MillisecondPart::Nanos(1)),
                ],
                "1y 1d 1h 1m 1s 1ms 1µs 1ns",
                "1 year 1 day 1 hour 1 minute 1 second 1 millisecond 1 microsecond 1 nanosecond",
            ),
            (
                [
                    Some(MillisecondPart::Years(2)),
                    Some(MillisecondPart::Days(3)),
                    Some(MillisecondPart::Hours(23)),
                    Some(MillisecondPart::Minutes(34)),
                    Some(MillisecondPart::Seconds(35)),
                    Some(MillisecondPart::Millis(360)),
                    Some(MillisecondPart::Micros(370)),
                    Some(MillisecondPart::Nanos(380)),
                ],
                "2y 3d 23h 34m 35s 360ms 370µs 380ns",
                "2 years 3 days 23 hours 34 minutes 35 seconds 360 milliseconds 370 microseconds 380 nanoseconds",
            ),
            (
                [
                    Some(MillisecondPart::Years(1)),
                    Some(MillisecondPart::Days(2)),
                    None,
                    Some(MillisecondPart::Minutes(3)),
                    None,
                    None,
                    None,
                    None,
                ],
                "1y 2d 3m",
                "1 year 2 days 3 minutes",
            ),
        ];

        for (test, exp_short, exp_long) in test_cases.iter() {
            let act = ms_parts_to_string(test, &MillisecondOption::default());
            assert_eq!(act, *exp_short);

            let act = ms_parts_to_string(test, &MillisecondOption::long());
            assert_eq!(act, *exp_long);
        }
    }
}
