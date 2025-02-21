/// The protocol for formatting to relative timestamp
pub trait RelativeFormatter {
    type Output;

    /// Provide a human-readable relative timestamp
    fn relative(&self) -> Self::Output;
}
