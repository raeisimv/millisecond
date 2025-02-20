use alloc::format;
use alloc::string::String;
use core::time::Duration;

use super::{formatter::RelativeFormatter, parser::RelativePart};
use crate::Millisecond;

impl RelativeFormatter for RelativePart {
    type Output = String;

    fn relative(&self) -> Self::Output {
        self.get_label_string()
    }
}

impl RelativeFormatter for Duration {
    type Output = String;

    fn relative(&self) -> Self::Output {
        let x: RelativePart = (*self).into();
        format!("{} ago", x.relative())
    }
}

impl RelativeFormatter for Millisecond {
    type Output = String;

    fn relative(&self) -> Self::Output {
        (**self).relative()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_duration_to_relative() {
        let dur = Duration::from_secs(30);
        assert_eq!(dur.relative(), "less than a minute ago");
    }
    #[test]
    fn should_convert_millisecond_to_relative() {
        let dur = Millisecond::from_secs(30);
        assert_eq!(dur.relative(), "less than a minute ago");
    }
}
