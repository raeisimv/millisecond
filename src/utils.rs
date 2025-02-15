use core::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ParsedUnitValue<T: Eq + Debug = u8, U: Eq + Debug = u64> {
    pub unit: T,
    pub total: U,
}

/// Convert the provided total seconds into its corresponding seconds (0-59) and the remaining total minutes.
///
/// > The unit is in the range of 0-59, while the total is the result of division by 60.
///
///## example
///```rust
/// use std::time::Duration;
/// use millisecond::utils::parse_secs;
///
/// let parsed = parse_secs(61);
/// assert_eq!(parsed.unit, 1_u8); // 1 second
/// assert_eq!(parsed.total, 1); // 1 minute
/// ```
///
#[inline]
pub fn parse_secs(total_secs: u64) -> ParsedUnitValue<u8, u64> {
    ParsedUnitValue {
        unit: (total_secs % 60) as u8,
        total: total_secs / 60,
    }
}

/// Convert the provided total minutes into its corresponding minutes (0-59) and the remaining total hours.
///
/// > The unit is in the range of 0-59, while the total is the result of division by 60.
///
///## example
///```rust
/// use std::time::Duration;
/// use millisecond::utils::parse_mins;
///
/// let parsed = parse_mins(61);
/// assert_eq!(parsed.unit, 1_u8); // 1 minute
/// assert_eq!(parsed.total, 1); // 1 hour
/// ```
///
#[inline]
pub fn parse_mins(total_mins: u64) -> ParsedUnitValue<u8, u64> {
    ParsedUnitValue {
        unit: (total_mins % 60) as u8,
        total: total_mins / 60,
    }
}

/// Convert the provided total hours into its corresponding hours (0-23) and the remaining total days.
///
/// > The unit is in the range of 0-23, while the total is the result of division by 24.
///
/// ## example
///```rust
/// use std::time::Duration;
/// use millisecond::utils::parse_hours;
///
/// let parsed = parse_hours(25);
/// assert_eq!(parsed.unit, 1_u8); // 1 hour
/// assert_eq!(parsed.total, 1); // 1 day
/// ```
///
#[inline]
pub fn parse_hours(total_hours: u64) -> ParsedUnitValue<u8, u64> {
    ParsedUnitValue {
        unit: (total_hours % 24) as u8,
        total: total_hours / 24,
    }
}

/// Convert the provided total days into its corresponding days (0-365) and the remaining total years.
///
/// > The unit is in the range of 0-365, while the total is the result of division by 365.
///
/// ## example
///```rust
/// use std::time::Duration;
/// use millisecond::utils::parse_days;
///
/// let parsed = parse_days(366);
/// assert_eq!(parsed.unit, 1_u16); // 1 day
/// assert_eq!(parsed.total, 1); // 1 year
/// ```
///
#[inline]
pub fn parse_days(total_days: u64) -> ParsedUnitValue<u16, u64> {
    ParsedUnitValue {
        unit: (total_days % 365) as u16,
        total: total_days / 365,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn should_parse_seconds_and_minutes() {
        let cases = vec![
            (0, ParsedUnitValue { unit: 0, total: 0 }),
            (1, ParsedUnitValue { unit: 1, total: 0 }),
            (59, ParsedUnitValue { unit: 59, total: 0 }),
            (60, ParsedUnitValue { unit: 0, total: 1 }),
            (61, ParsedUnitValue { unit: 1, total: 1 }),
            (119, ParsedUnitValue { unit: 59, total: 1 }),
            (120, ParsedUnitValue { unit: 0, total: 2 }),
            (121, ParsedUnitValue { unit: 1, total: 2 }),
            (
                3599,
                ParsedUnitValue {
                    unit: 59,
                    total: 59,
                },
            ),
            (3600, ParsedUnitValue { unit: 0, total: 60 }),
            (3601, ParsedUnitValue { unit: 1, total: 60 }),
        ];

        for (actual, expected) in cases {
            let sec_res = parse_secs(actual);
            let min_res = parse_mins(actual);
            assert_eq!(
                sec_res, expected,
                "Seconds -> Expected:{:?} Got:{:?}",
                expected, sec_res
            );
            assert_eq!(
                min_res, expected,
                "Minutes -> Expected:{:?} Got:{:?}",
                expected, min_res
            );
        }
    }

    #[test]
    fn should_parse_hours() {
        let cases = vec![
            (0, ParsedUnitValue { unit: 0, total: 0 }),
            (1, ParsedUnitValue { unit: 1, total: 0 }),
            (23, ParsedUnitValue { unit: 23, total: 0 }),
            (24, ParsedUnitValue { unit: 0, total: 1 }),
            (25, ParsedUnitValue { unit: 1, total: 1 }),
            (47, ParsedUnitValue { unit: 23, total: 1 }),
            (48, ParsedUnitValue { unit: 0, total: 2 }),
            (49, ParsedUnitValue { unit: 1, total: 2 }),
            (
                575,
                ParsedUnitValue {
                    unit: 23,
                    total: 23,
                },
            ),
            (576, ParsedUnitValue { unit: 0, total: 24 }),
            (577, ParsedUnitValue { unit: 1, total: 24 }),
        ];

        for (actual, expected) in cases {
            let res = parse_hours(actual);
            assert_eq!(res, expected);
        }
    }
    #[test]
    fn should_parse_days() {
        let cases = vec![
            (0, ParsedUnitValue { unit: 0, total: 0 }),
            (1, ParsedUnitValue { unit: 1, total: 0 }),
            (
                364,
                ParsedUnitValue {
                    unit: 364,
                    total: 0,
                },
            ),
            (365, ParsedUnitValue { unit: 0, total: 1 }),
            (366, ParsedUnitValue { unit: 1, total: 1 }),
        ];

        for (actual, expected) in cases {
            let res = parse_days(actual);
            assert_eq!(res, expected);
        }
    }
}
