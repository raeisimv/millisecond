use crate::duration::Weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};

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
