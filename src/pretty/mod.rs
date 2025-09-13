mod formatter;
pub mod parser;
pub mod splitter;
mod text_gen;
pub mod utils;

pub use formatter::*;
pub use parser::*;
pub use splitter::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_separate_and_combine_milliseconds() {
        assert_eq!(Millisecond::from_millis(1100).pretty(), "1s 100ms");
        assert_eq!(
            Millisecond::from_millis(1100).pretty_with(MillisecondOption {
                seconds: SecondsOptions::Combine,
                ..Default::default()
            }),
            "1.1s"
        );
    }
}
