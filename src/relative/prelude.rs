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
        x.relative()
    }
}

impl RelativeFormatter for Millisecond {
    type Output = String;

    fn relative(&self) -> Self::Output {
        (**self).relative()
    }
}
