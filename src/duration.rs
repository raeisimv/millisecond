use crate::duration::Weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
use core::time::Duration;

/// A Strongly Typed definition for day of week
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    pub fn to_str(&self) -> &'static str {
        match self {
            Sunday => "Sunday",
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
        }
    }
}

impl From<Weekday> for u8 {
    fn from(value: Weekday) -> Self {
        match value {
            Sunday => 0,
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
        }
    }
}

impl From<Duration> for Weekday {
    /// Converts a duration into corresponding weekday.
    ///
    /// ## Note
    /// This function assumes the duration is from Linux Epoch, which starts at (Thursday, January 1, 1970)
    ///
    /// ### Link
    /// I've posted it on [StackOverflow](https://stackoverflow.com/questions/66181608/how-can-i-get-the-current-weekday-in-rust-using-the-chrono-crate/79461838#79461838)
    fn from(value: Duration) -> Self {
        match (value.as_secs() / 86400) % 7 {
            0 => Thursday, // zero is Thursday
            1 => Friday,
            2 => Saturday,
            3 => Sunday,
            4 => Monday,
            5 => Tuesday,
            6 => Wednesday,
            _ => panic!("Invalid duration - never happens"),
        }
    }
}

/// Duration parsing errors
pub enum DurationError {
    /// The is not a valid week day
    InvalidConversion,
}

impl TryFrom<u8> for Weekday {
    type Error = DurationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Sunday),
            1 => Ok(Monday),
            2 => Ok(Tuesday),
            3 => Ok(Wednesday),
            4 => Ok(Thursday),
            5 => Ok(Friday),
            6 => Ok(Saturday),
            _ => Err(DurationError::InvalidConversion),
        }
    }
}
