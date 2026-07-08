use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Out of bounds")]
    OutOfBounds,

    #[error("Invalid alignment")]
    InvalidAlignment,

    #[error("Collection length overflow")]
    CollectionLengthOverflow,

    #[error("Collection length is negative")]
    CollectionLengthNegative,
}
