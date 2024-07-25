pub trait Discriminable {
    type Discriminant;

    fn discriminant(&self) -> Self::Discriminant;
}
