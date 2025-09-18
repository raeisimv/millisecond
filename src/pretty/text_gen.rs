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
pub(crate) fn get_part_colon_label(part: &MillisecondPart, _opt: &MillisecondOption) -> String {
    match *part {
        MillisecondPart::Years(x) => format!("{x:04}"),
        MillisecondPart::Days(x) => format!("{x:03}"),
        MillisecondPart::Hours(x) => format!("{x:02}"),
        MillisecondPart::Minutes(x) => format!("{x:01}"),
        MillisecondPart::Seconds(x) => format!("{x:02}"),
        MillisecondPart::SecondsAndMs(secs, millis) => combine_secs_and_millis_colon(secs, millis),
        MillisecondPart::Millis(x) => format!("{x:03}"),
        MillisecondPart::Micros(x) => format!("{x:03}"),
        MillisecondPart::Nanos(x) => format!("{x:03}"),
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
fn combine_secs_and_millis_colon(secs: u8, millis: u16) -> String {
    let secs_str = format!("{secs:02}");
    if millis == 0 {
        return secs_str;
    }

    let millis_str = format!("{millis:03}");
    let precision = 1;
    format!("{}.{}", secs_str, &millis_str[..precision])
}
