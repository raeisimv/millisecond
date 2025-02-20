pub trait RelativeFormatter {
    type Output;

    fn relative(&self) -> Self::Output;
}
