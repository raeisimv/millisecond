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
                        precision,
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
            (1100, 1, "1.1s"),
            (1100, 2, "1.10s"),
            (1100, 3, "1.100s"),
            (1100, 0, "1.1s"),   // clamp
            (1100, 4, "1.100s"), // clamp
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
}
