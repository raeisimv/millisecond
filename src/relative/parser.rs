use alloc::string::{String, ToString};
use core::time::Duration;

const SECONDS: &str = "less than a minute";
const MINUTE: &str = "about a minute";
const MINUTES: &str = "%d minutes";
const HOUR: &str = "about an hour";
const HOURS: &str = "about %d hours";
const DAY: &str = "a day";
const DAYS: &str = "%d days";
const MONTH: &str = "about a month";
const MONTHS: &str = "%d months";
const YEAR: &str = "about a year";
const YEARS: &str = "%d years";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativePart {
    FewSecs,
    AboutAMinute,
    Minutes(u64),
    AboutAnHour,
    Hours(u64),
    AboutADay,
    Days(u64),
    AboutAMonth,
    Months(u64),
    AboutAYear,
    Years(u64),
}
impl RelativePart {
    pub fn from_secs(secs: u64) -> Self {
        match secs / 31536000 {
            0 => match secs / 2592000 {
                0 => match secs / 86400 {
                    0 => match secs / 3600 {
                        0 => match secs / 60 {
                            0 => RelativePart::FewSecs,
                            1 => RelativePart::AboutAMinute,
                            x => RelativePart::Minutes(x),
                        },
                        1 => RelativePart::AboutAnHour,
                        x => RelativePart::Hours(x),
                    },
                    1 => RelativePart::AboutADay,
                    x => RelativePart::Days(x),
                },
                1 => RelativePart::AboutAMonth,
                x => RelativePart::Months(x),
            },
            1 => RelativePart::AboutAYear,
            x => RelativePart::Years(x),
        }
    }
    pub fn get_label(&self) -> (Option<u64>, &'static str) {
        match self {
            RelativePart::FewSecs => (None, SECONDS),
            RelativePart::AboutAMinute => (None, MINUTE),
            RelativePart::Minutes(x) => (Some(*x), MINUTES),
            RelativePart::AboutAnHour => (None, HOUR),
            RelativePart::Hours(x) => (Some(*x), HOURS),
            RelativePart::AboutADay => (None, DAY),
            RelativePart::Days(x) => (Some(*x), DAYS),
            RelativePart::AboutAMonth => (None, MONTH),
            RelativePart::Months(x) => (Some(*x), MONTHS),
            RelativePart::AboutAYear => (None, YEAR),
            RelativePart::Years(x) => (Some(*x), YEARS),
        }
    }
    pub fn get_label_string(&self) -> String {
        let (val, label) = self.get_label();
        if let Some(x) = val {
            label.replace("%d", &x.to_string())
        } else {
            label.into()
        }
    }
}

impl From<Duration> for RelativePart {
    fn from(value: Duration) -> Self {
        Self::from_secs(value.as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_say_less_than_a_minute() {
        assert_eq!(
            RelativePart::from_secs(0).get_label_string(),
            "less than a minute"
        );
        assert_eq!(
            RelativePart::from_secs(30).get_label_string(),
            "less than a minute"
        );
        assert_eq!(
            RelativePart::from_secs(59).get_label_string(),
            "less than a minute"
        );
    }

    #[test]
    fn should_say_about_a_minute() {
        assert_eq!(
            RelativePart::from_secs(60).get_label_string(),
            "about a minute"
        );
        assert_eq!(
            RelativePart::from_secs(119).get_label_string(),
            "about a minute"
        );
    }

    #[test]
    fn should_say_n_minutes() {
        assert_eq!(RelativePart::from_secs(120).get_label_string(), "2 minutes");
        assert_eq!(RelativePart::from_secs(300).get_label_string(), "5 minutes");
        assert_eq!(
            RelativePart::from_secs(3599).get_label_string(),
            "59 minutes"
        );
    }

    #[test]
    fn should_say_about_an_hour() {
        assert_eq!(
            RelativePart::from_secs(3600).get_label_string(),
            "about an hour"
        );
        assert_eq!(
            RelativePart::from_secs(7199).get_label_string(),
            "about an hour"
        );
    }

    #[test]
    fn should_say_about_n_hours() {
        assert_eq!(
            RelativePart::from_secs(7200).get_label_string(),
            "about 2 hours"
        );
        assert_eq!(
            RelativePart::from_secs(18000).get_label_string(),
            "about 5 hours"
        );
        assert_eq!(
            RelativePart::from_secs(86399).get_label_string(),
            "about 23 hours"
        );
    }

    #[test]
    fn should_say_a_day() {
        assert_eq!(RelativePart::from_secs(86400).get_label_string(), "a day");
        assert_eq!(RelativePart::from_secs(172799).get_label_string(), "a day");
    }

    #[test]
    fn should_says_n_days() {
        assert_eq!(RelativePart::from_secs(172800).get_label_string(), "2 days");
        assert_eq!(RelativePart::from_secs(604800).get_label_string(), "7 days");
        assert_eq!(
            RelativePart::from_secs(2591999).get_label_string(),
            "29 days"
        );
    }

    #[test]
    fn should_say_about_a_month() {
        assert_eq!(
            RelativePart::from_secs(2592000).get_label_string(),
            "about a month"
        );
        assert_eq!(
            RelativePart::from_secs(5183999).get_label_string(),
            "about a month"
        );
    }

    #[test]
    fn should_say_n_months() {
        assert_eq!(
            RelativePart::from_secs(5184000).get_label_string(),
            "2 months"
        );
        assert_eq!(
            RelativePart::from_secs(15552000).get_label_string(),
            "6 months"
        );
        assert_eq!(
            RelativePart::from_secs(31535999).get_label_string(),
            "12 months"
        );
    }

    #[test]
    fn should_say_about_a_year() {
        assert_eq!(
            RelativePart::from_secs(31536000).get_label_string(),
            "about a year"
        );
        assert_eq!(
            RelativePart::from_secs(63071999).get_label_string(),
            "about a year"
        );
    }

    #[test]
    fn test_years() {
        assert_eq!(
            RelativePart::from_secs(63072000).get_label_string(),
            "2 years"
        );
        assert_eq!(
            RelativePart::from_secs(157680000).get_label_string(),
            "5 years"
        );
        assert_eq!(
            RelativePart::from_secs(315360000).get_label_string(),
            "10 years"
        );
    }

    #[test]
    fn should_convert_from_duration() {
        let dur = core::time::Duration::ZERO;
        let rel: RelativePart = dur.into();
        assert_eq!(rel.get_label_string(), "less than a minute");

        let rel: RelativePart = core::time::Duration::from_secs(61).into();
        assert_eq!(rel.get_label_string(), "about a minute");
    }
}
