use core::fmt::{Display, Formatter};

use crate::Millisecond;

#[derive(Debug)]
pub enum MillisecondPart {
    Years(u64),
    Days(u16),
    Hours(u8),
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
            MillisecondPart::Years(x) => { format!("{x}y") }
            MillisecondPart::Days(x) => { format!("{x}d") }
            MillisecondPart::Hours(x) => { format!("{x}h") }
            MillisecondPart::Minutes(x) => { format!("{x}m") }
            MillisecondPart::Seconds(x) => { format!("{x}s") }
            MillisecondPart::Millis(x) => { format!("{x}ms") }
            MillisecondPart::SecsAndMillis(x, y) => { format!("{x}.{y}s") }
            MillisecondPart::Micros(x) => { format!("{x}µs") }
            MillisecondPart::Nanos(x) => { format!("{x}ns") }
        }
    }
    pub fn to_long_string(&self) -> String {
        match self {
            MillisecondPart::Years(x) => { with_pluralization(x, "year", 1) }
            MillisecondPart::Days(x) => { with_pluralization(x, "day", 1) }
            MillisecondPart::Hours(x) => { with_pluralization(x, "hour", 1) }
            MillisecondPart::Minutes(x) => { with_pluralization(x, "minute", 1) }
            MillisecondPart::Seconds(x) => { with_pluralization(x, "second", 1) }
            MillisecondPart::Millis(x) => { with_pluralization(x, "millisecond", 1) }
            MillisecondPart::SecsAndMillis(x, y) => { format!("{x}.{y} seconds") }
            MillisecondPart::Micros(x) => { with_pluralization(x, "microsecond", 1) }
            MillisecondPart::Nanos(x) => { with_pluralization(x, "nanosecond", 1) }
        }
    }
    pub fn from_millisecond(ms: &Millisecond) -> Vec<MillisecondPart> {
        Self::from_millisecond_with_option(ms, true)
    }
    pub fn from_millisecond_with_option(ms: &Millisecond, merge_secs_and_millis: bool) -> Vec<MillisecondPart> {
        let mut v = vec![];
        if ms.years > 0 {
            v.push(MillisecondPart::Years(ms.years));
        }
        if ms.days > 0 {
            v.push(MillisecondPart::Days(ms.days));
        }
        if ms.hours > 0 {
            v.push(MillisecondPart::Hours(ms.hours));
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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