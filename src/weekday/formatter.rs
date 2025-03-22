use crate::prelude::Weekday;

/// The protocol for types which would convert to weekday. e.g. Duration and Time
pub trait WeekdayConversion {
    /// Returns the weekday for the given value
    fn weekday(&self) -> Weekday;

    /// Return the weekday in string for the given value
    fn weekday_str(&self) -> &'static str {
        self.weekday().to_str()
    }
}
