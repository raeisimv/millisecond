use alloc::{format, string::String};

use crate::{MillisecondOption, pretty::MillisecondPart};

pub(crate) fn get_part_long_label(part: &MillisecondPart, _opt: &MillisecondOption) -> String {
    match part {
        MillisecondPart::Years(x) => {
            if *x != 1 {
                format!("{x} years")
            } else {
                format!("{x} year")
            }
        }
        MillisecondPart::Days(x) => {
            if *x != 1 {
                format!("{x} days")
            } else {
                format!("{x} day")
            }
        }
        MillisecondPart::Hours(x) => {
            if *x != 1 {
                format!("{x} hours")
            } else {
                format!("{x} hour")
            }
        }
        MillisecondPart::Minutes(x) => {
            if *x != 1 {
                format!("{x} minutes")
            } else {
                format!("{x} minute")
            }
        }
        MillisecondPart::Seconds(x) => {
            if *x != 1 {
                format!("{x} seconds")
            } else {
                format!("{x} second")
            }
        }
        MillisecondPart::Millis(x) => {
            if *x != 1 {
                format!("{x} milliseconds")
            } else {
                format!("{x} millisecond")
            }
        }
        MillisecondPart::Micros(x) => {
            if *x != 1 {
                format!("{x} microseconds")
            } else {
                format!("{x} microsecond")
            }
        }
        MillisecondPart::Nanos(x) => {
            if *x != 1 {
                format!("{x} nanoseconds")
            } else {
                format!("{x} nanosecond")
            }
        }
    }
}
pub(crate) fn get_part_short_label(part: &MillisecondPart, _opt: &MillisecondOption) -> String {
    match part {
        MillisecondPart::Years(x) => {
            format!("{x}y")
        }
        MillisecondPart::Days(x) => {
            format!("{x}d")
        }
        MillisecondPart::Hours(x) => {
            format!("{x}h")
        }
        MillisecondPart::Minutes(x) => {
            format!("{x}m")
        }
        MillisecondPart::Seconds(x) => {
            format!("{x}s")
        }
        MillisecondPart::Millis(x) => {
            format!("{x}ms")
        }
        MillisecondPart::Micros(x) => {
            format!("{x}µs")
        }
        MillisecondPart::Nanos(x) => {
            format!("{x}ns")
        }
    }
}
