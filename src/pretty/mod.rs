mod formatter;
pub mod parser;
pub mod splitter;
mod text_gen;
pub mod utils;

pub use formatter::*;
pub use parser::*;
pub use splitter::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_separate_and_combine_milliseconds() {
        assert_eq!(Millisecond::from_millis(1100).pretty(), "1s 100ms");
        assert_eq!(
            Millisecond::from_millis(1100).pretty_with(MillisecondOption {
                seconds: SecondsOptions::Combine,
                ..Default::default()
            }),
            "1.1s"
        );
    }
    #[test]
    fn should_combine_seconds_and_milliseconds() {
        let cases = [
            (1100, "1.1s"),
            (100, "0.1s"),
            (1000, "1.0s"),
            (1001, "1.0s"),
            (1011, "1.0s"),
            (1111, "1.1s"),
            (0, ""),
            (60_000, "1m"),
            (999, "0.9s"),
            (1999, "1.9s"),
            (59999, "59.9s"),
        ];
        for (millis, expected) in cases {
            assert_eq!(
                Millisecond::from_millis(millis).pretty_with(MillisecondOption {
                    seconds: SecondsOptions::Combine,
                    ..Default::default()
                }),
                expected
            );
        }
    }
    #[test]
    fn should_combine_seconds_and_milliseconds_with_fixed_width() {
        let cases = [
            (1100, 1, "01.1s"),
            (1100, 2, "01.10s"),
            (1100, 3, "01.100s"),
            (100, 3, "00.100s"),
            (1000, 3, "01.000s"),
            (1011, 3, "01.011s"),
            (1001, 1, "01.0s"),
            (1001, 2, "01.00s"),
            (1001, 3, "01.001s"),
            (1111, 1, "01.1s"),
            (1111, 2, "01.11s"),
            (1111, 3, "01.111s"),
            (0, 3, ""),
            (60_000, 3, "1m"),
            (999, 3, "00.999s"),
            (1999, 3, "01.999s"),
            (59999, 3, "59.999s"),
        ];
        for (millis, precision, expected) in cases {
            assert_eq!(
                Millisecond::from_millis(millis).pretty_with(MillisecondOption {
                    seconds: SecondsOptions::CombineWith {
                        precision: Some(precision),
                        fixed_width: true
                    },
                    ..Default::default()
                }),
                expected
            );
        }
    }
    #[test]
    fn should_combine_seconds_and_milliseconds_with_precision() {
        let cases = [
            (1100, None, "1.1s"),
            (1100, Some(0), "1s"),
            (1100, Some(1), "1.1s"),
            (1100, Some(2), "1.10s"),
            (1100, Some(3), "1.100s"),
            (1100, Some(4), "1.100s"), // clamp
        ];
        for (millis, precision, expected) in cases {
            assert_eq!(
                Millisecond::from_millis(millis).pretty_with(MillisecondOption {
                    seconds: SecondsOptions::CombineWith {
                        precision,
                        fixed_width: false
                    },
                    ..Default::default()
                }),
                expected
            );
        }
    }
    #[test]
    fn should_format_with_colon_notation() {
        let cases = [
            // Default formats
            (1000, None, "0:01"),
            (1543, None, "0:01.5"),
            (1000 * 60, None, "1:00"),
            (1000 * 90, None, "1:30"),
            (95_543, None, "1:35.5"),
            ((1000 * 60 * 10) + 543, None, "10:00.5"),
            ((1000 * 60 * 59) + (1000 * 59) + 543, None, "59:59.5"),
            (
                (1000 * 60 * 60 * 15) + (1000 * 60 * 59) + (1000 * 59) + 543,
                None,
                "15:59:59.5",
            ),
        ];
        for (millis, precision, expected) in cases {
            let act = Millisecond::from_millis(millis).pretty_with(MillisecondOption {
                seconds: SecondsOptions::CombineWith {
                    precision,
                    fixed_width: false,
                },
                ..MillisecondOption::colon()
            });
            assert_eq!(act, expected,);
        }
    }
    #[test]
    fn should_count_units() {
        let cases = [
            (1000 * 60, 0, "1m"),
            (1000 * 60, 1, "1m"),
            (1000 * 60 * 67, 1, "1h"),
            (1000 * 60 * 67, 2, "1h 7m"),
            (1000 * 60 * 67 * 24 * 465, 1, "1y"),
            (1000 * 60 * 67 * 24 * 465, 2, "1y 154d"),
            (1000 * 60 * 67 * 24 * 465, 3, "1y 154d 6h"),
        ];
        for (millis, unit_count, expected) in cases {
            let opt = MillisecondOption {
                unit_count: Some(unit_count),
                ..Default::default()
            };
            let act = Millisecond::from_millis(millis).pretty_with(opt);
            assert_eq!(act, expected);
        }
    }
    #[test]
    fn should_separate_seconds_and_milliseconds() {
        let cases = [
            (1100, SecondsOptions::Combine, "1.1s"),
            (1100, SecondsOptions::Separate, "1s 100ms"),
        ];
        for (millis, seconds_option, expected) in cases {
            let opt = MillisecondOption {
                seconds: seconds_option,
                ..Default::default()
            };
            let act = Millisecond::from_millis(millis).pretty_with(opt);
            assert_eq!(act, expected);
        }
    }
    #[test]
    fn should_separate_seconds_and_millis_work_with_format_sub_millis() {
        let cases = [
            (1_010_340_067, "1s 10ms 340µs 67ns"),
            ((60 * 1_000_000_000) + 34_000_000 + 5, "1m 34ms 5ns"),
        ];
        for (millis, expected) in cases {
            let opt = MillisecondOption {
                seconds: SecondsOptions::Separate,
                format_sub_milliseconds: true,
                ..Default::default()
            };
            let act = Millisecond::from_nanos(millis).pretty_with(opt);
            assert_eq!(act, expected);
        }
    }
    #[test]
    fn should_long_option_works_with_unit_count() {
        let cases = [
            (1000 * 60, 1, "1 minute"),
            (1000 * 60 * 67, 1, "1 hour"),
            (1000 * 60 * 67, 2, "1 hour 7 minutes"),
            (1000 * 60 * 67 * 24 * 465, 1, "1 year"),
            (1000 * 60 * 67 * 24 * 465, 2, "1 year 154 days"),
            (1000 * 60 * 67 * 24 * 465, 3, "1 year 154 days 6 hours"),
        ];
        for (millis, unit_count, expected) in cases {
            let opt = MillisecondOption {
                unit_count: Some(unit_count),
                ..MillisecondOption::long()
            };
            let act = Millisecond::from_millis(millis).pretty_with(opt);
            assert_eq!(act, expected);
        }
    }
    #[test]
    fn should_customize_separator() {
        use crate::pretty::OutputFormat::*;
        use crate::pretty::Separator::*;

        let cases = [
            (1000 * 60 * 67 * 24 * 465, Short, Default, "1y 154d 6h"),
            (1000 * 60 * 67 * 24 * 465, Colon, Default, "1:154:06:00:00"), // based on the format flag
            (1000 * 60 * 67 * 24 * 465, Short, Space, "1y 154d 6h"),
            (1000 * 60 * 67 * 24 * 465, Colon, Space, "1 154 06 00 00"), // overrides colon with space
            (1000 * 60 * 67 * 24 * 465, Short, None, "1y154d6h"),
            (1000 * 60 * 67 * 24 * 465, Colon, Dash, "1-154-06-00-00"), // overrides colon with a dash
            (1000 * 60 * 67 * 24 * 465, Short, Dash, "1y-154d-6h"),
            (1000 * 60 * 67 * 24 * 465, Short, Custom("%"), "1y%154d%6h"),
        ];
        for (millis, format, separator, expected) in cases {
            let opt = MillisecondOption {
                format,
                separator,
                ..MillisecondOption::default()
            };
            let act = Millisecond::from_millis(millis).pretty_with(opt);
            assert_eq!(act, expected);
        }
    }
    #[test]
    fn should_cooperate_dominant_only_flag_with_the_long_flag() {
        let cases = [
            (1000, "1 second"),
            (1000 + 400, "1 second"),
            ((1000 * 2) + 400, "2 seconds"),
            (1000 * 5, "5 seconds"),
            (1000 * 55, "55 seconds"),
            (1000 * 67, "1 minute"),
            (1000 * 60 * 5, "5 minutes"),
            (1000 * 60 * 67, "1 hour"),
            (1000 * 60 * 60 * 12, "12 hours"),
            (1000 * 60 * 60 * 40, "1 day"),
            (1000 * 60 * 60 * 999, "41 days"),
            (1000 * 60 * 60 * 24 * 465, "1 year"),
            (1000 * 60 * 67 * 24 * 750, "2 years"),
        ];
        for (i, (millis, expected)) in cases.into_iter().enumerate() {
            let opt = MillisecondOption {
                format: OutputFormat::Long,
                dominant_only: true,
                ..MillisecondOption::default()
            };
            let act = Millisecond::from_millis(millis).pretty_with(opt);
            assert_eq!(act, expected, "{i}, millis: {millis}");
        }
    }
}
