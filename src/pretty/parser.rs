use alloc::{string::String, vec::Vec};
use core::time::Duration;

use crate::pretty::{
    MillisecondOption, OutputFormat, SecondsOptions,
    text_gen::{get_part_long_label, get_part_short_label},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MillisecondPart {
    Years(u64),
    Days(u64),
    Hours(u8),
    Minutes(u8),
    Seconds(u8),
    SecondsAndMs(u8, u16),
    Millis(u16),
    Micros(u16),
    Nanos(u16),
}

/// This function parse the given Duration into crate's language for further use.
pub fn parse_duration(dur: &Duration, opt: &MillisecondOption) -> [Option<MillisecondPart>; 8] {
    let mut parts = [None; 8];
    if dur.is_zero() {
        return parts;
    }

    use crate::pretty::utils::ParsedUnitValue;

    let total_nanos = dur.subsec_nanos();
    let format_sub_millis = opt.format_sub_milliseconds;
    if total_nanos > 0 {
        if format_sub_millis {
            let nanos = total_nanos % 1_000;
            if nanos > 0 {
                parts[7] = Some(MillisecondPart::Nanos(nanos as _));
            }
        }

        let micros = dur.subsec_micros();
        if micros > 0 {
            if format_sub_millis {
                let micros = micros % 1000;
                if micros > 0 {
                    parts[6] = Some(MillisecondPart::Micros((micros % 1000) as _));
                }
            }

            let millis = dur.subsec_millis();
            if millis > 0 {
                parts[5] = Some(MillisecondPart::Millis(millis as _));
            }
        }
    }

    let ParsedUnitValue { unit: secs, total } = ParsedUnitValue::parse_secs(dur.as_secs());
    if secs > 0 {
        parts[4] = Some(MillisecondPart::Seconds(secs));
    }
    if total > 0 {
        let ParsedUnitValue { unit: mins, total } = ParsedUnitValue::parse_mins(total);
        if mins > 0 {
            parts[3] = Some(MillisecondPart::Minutes(mins));
        }

        if total > 0 {
            let ParsedUnitValue { unit: hours, total } = ParsedUnitValue::parse_hours(total);
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
                    } = ParsedUnitValue::parse_days(total);
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

    match opt.seconds {
        SecondsOptions::Combine | SecondsOptions::CombineWith { .. } => {
            let secs = if let Some(MillisecondPart::Seconds(secs)) = parts[4] {
                parts[4] = None;
                secs
            } else {
                0
            };
            let millis = if let Some(MillisecondPart::Millis(millis)) = parts[5] {
                parts[5] = None;
                millis
            } else {
                0
            };
            if secs != 0 || millis != 0 {
                parts[4] = Some(MillisecondPart::SecondsAndMs(secs, millis));
            }
        }
        SecondsOptions::Separate => {
            // do nothing it is already separated
        }
        SecondsOptions::Hide => {
            parts[4] = None;
            parts[5] = None;
        }
    }

    parts
}

impl MillisecondPart {
    /// Converts the crate's language to human-readable string
    pub fn get_label(&self, opt: &MillisecondOption) -> String {
        match opt.format {
            OutputFormat::Short => get_part_short_label(self, opt),
            OutputFormat::Long => get_part_long_label(self, opt),
            OutputFormat::Colon => todo!(),
        }
    }
}

/// Converts the crate's language tokens into the human-readable string
pub fn ms_parts_to_string(parts: &[Option<MillisecondPart>; 8], opt: &MillisecondOption) -> String {
    let take = if opt.dominant_only {
        1
    } else {
        8 // or infinity
    };
    parts
        .iter()
        .filter(|x| x.is_some())
        .take(take)
        .map(|x| x.unwrap().get_label(opt))
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

        let opt = MillisecondOption::backward_compatible();
        for (dur, exp) in cases.iter() {
            let parts = parse_duration(dur, &opt);
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

        let opt = MillisecondOption::backward_compatible();
        for (dur, exp) in cases.iter() {
            let parts = parse_duration(dur, &opt);
            assert_eq!(parts, *exp);
        }
    }
    #[test]
    fn should_convert_to_string() {
        let test_cases = [
            ([None, None, None, None, None, None, None, None], "", ""),
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

        let opt_short = MillisecondOption::backward_compatible();
        let opt_long = MillisecondOption {
            format: OutputFormat::Long,
            ..opt_short
        };

        for (test, exp_short, exp_long) in test_cases.iter() {
            let act = ms_parts_to_string(test, &opt_short);
            assert_eq!(act, *exp_short);

            let act = ms_parts_to_string(test, &opt_long);
            assert_eq!(act, *exp_long);
        }
    }
    #[test]
    fn should_obtain_days_instead_of_years() {
        let test_cases = [
            (Duration::from_secs((365 + 1) * 24 * 60 * 60), "366d"),
            (Duration::from_secs(2 * 24 * 60 * 60), "2d"),
            (Duration::from_secs(23 * 60 * 60), "23h"),
        ];

        let opt = MillisecondOption {
            days_instead_of_years: true,
            ..MillisecondOption::backward_compatible()
        };

        for (test, exp) in test_cases.iter() {
            let act = ms_parts_to_string(&parse_duration(test, &opt), &opt);
            assert_eq!(&act, exp);
        }
    }
    #[test]
    fn should_display_dominant_part_only() {
        #[allow(clippy::identity_op)]
        let test_cases = [
            (Duration::from_secs((365 + 0) * 24 * 60 * 60), "1y"),
            (Duration::from_secs((365 + 1) * 24 * 60 * 60), "1y"),
            (Duration::from_secs((24 + 0) * 60 * 60), "1d"),
            (Duration::from_secs((24 + 1) * 60 * 60), "1d"),
            (Duration::from_secs((60 + 0) * 60), "1h"),
            (Duration::from_secs((60 + 1) * 60), "1h"),
            (
                Duration::from_secs((365 * 24 * 60 * 60) + (23 * 60 * 60)),
                "1y",
            ),
            (Duration::from_millis(2_100), "2s"),
            (Duration::from_millis(100), "100ms"),
        ];

        for (test, exp) in test_cases.iter() {
            let opt = MillisecondOption {
                dominant_only: true,
                ..MillisecondOption::backward_compatible()
            };

            let act = ms_parts_to_string(&parse_duration(test, &opt), &opt);
            assert_eq!(&act, exp);
        }

        let opt = MillisecondOption {
            dominant_only: true,
            days_instead_of_years: true,
            ..MillisecondOption::backward_compatible()
        };
        assert_eq!(
            "366d",
            ms_parts_to_string(
                &parse_duration(&Duration::from_secs((365 + 1) * 24 * 60 * 60), &opt),
                &opt
            )
        );
    }
}
