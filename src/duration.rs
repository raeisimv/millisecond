use crate::duration::Weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
use core::time::Duration;

/// The protocol for types which would convert to weekday. e.g. Duration and Time
pub trait WeekdayConversion {
    /// Returns the weekday for the given value
    fn weekday(&self) -> Weekday;

    /// Return the weekday in string for the given value
    fn weekday_str(&self) -> &'static str {
        self.weekday().to_str()
    }
}

impl WeekdayConversion for Duration {
    fn weekday(&self) -> Weekday {
        Weekday::from(*self)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    #[test]
    fn should_convert_to_epoch() {
        let dur: Weekday = Duration::from_secs(0).into();
        assert_eq!(dur.to_str(), "Thursday");
    }

    #[test]
    fn should_convert_duration_to_weekday() {
        let dur = Duration::from_secs(0);
        assert_eq!(dur.weekday().to_str(), "Thursday");
        assert_eq!(dur.weekday_str(), "Thursday");
    }

    #[test]
    fn should_convert_to_weekday() {
        let dur: Weekday = Duration::from_secs(0).into();
        assert_eq!(dur.to_str(), "Thursday");
        let dur: Weekday = Duration::from_secs(24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Friday");
        let dur: Weekday = Duration::from_secs(2 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Saturday");
        let dur: Weekday = Duration::from_secs(3 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Sunday");
        let dur: Weekday = Duration::from_secs(4 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Monday");
        let dur: Weekday = Duration::from_secs(5 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Tuesday");
        let dur: Weekday = Duration::from_secs(6 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Wednesday");
        let dur: Weekday = Duration::from_secs(7 * 24 * 60 * 60).into();
        assert_eq!(dur.to_str(), "Thursday");
    }
}
