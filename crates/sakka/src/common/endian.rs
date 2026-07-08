/// The byte order used when encoding and decoding primitive values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// Native system endianness.
    ///
    /// This uses the endianness of the target platform. It is not portable and
    /// should generally only be used for machine-local data.
    Native,

    /// Little-endian representation.
    Little,

    /// Big-endian representation.
    Big,
}
