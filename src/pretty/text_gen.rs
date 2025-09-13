use alloc::{format, string::String};

use crate::{MillisecondOption, pretty::MillisecondPart};

pub(crate) fn get_part_long_label(part: &MillisecondPart, opt: &MillisecondOption) -> String {
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
        MillisecondPart::SecondsAndMs(secs, millis) => {
            format!("{} seconds", combine_secs_and_millis(*secs, *millis, opt))
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
pub(crate) fn get_part_short_label(part: &MillisecondPart, opt: &MillisecondOption) -> String {
    match part {
        MillisecondPart::Years(x) => format!("{x}y"),
        MillisecondPart::Days(x) => format!("{x}d"),
        MillisecondPart::Hours(x) => format!("{x}h"),
        MillisecondPart::Minutes(x) => format!("{x}m"),
        MillisecondPart::Seconds(x) => format!("{x}s"),
        MillisecondPart::SecondsAndMs(secs, millis) => {
            format!("{}s", combine_secs_and_millis(*secs, *millis, opt))
        }
        MillisecondPart::Millis(x) => format!("{x}ms"),
        MillisecondPart::Micros(x) => format!("{x}µs"),
        MillisecondPart::Nanos(x) => format!("{x}ns"),
    }
}

fn combine_secs_and_millis(secs: u8, millis: u16, opt: &MillisecondOption) -> String {
    let secs_str = if opt.seconds.is_fixed_width() {
        format!("{secs:02}")
    } else {
        format!("{}", secs)
    };

    let millis_str = format!("{millis:03}");
    let precision = opt.seconds.precision() as _;
    format!(
        "{}.{}",
        secs_str,
        millis_str.get(..precision).unwrap_or("0")
    )
}
