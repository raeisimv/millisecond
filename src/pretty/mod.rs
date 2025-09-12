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
    fn should_have_separate_millisecond() {
        assert_eq!(Millisecond::from_millis(1100).pretty(), "1.1s");
        assert_eq!(
            Millisecond::from_millis(1100).pretty_with(MillisecondOption {
                separate_milliseconds: true,
                ..Default::default()
            }),
            "1s 100ms"
        );
    }
}
