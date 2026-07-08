use crate::Error;

/// Returns the number of padding bytes needed to align the given position to the given alignment.
///
/// If the alignment is invalid (i.e., zero or not a power of two), an [Error::InvalidAlignment]
/// is returned.
#[inline]
pub fn alignment_padding(position: usize, alignment: usize) -> Result<usize, Error> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return Err(Error::InvalidAlignment);
    }

    Ok((alignment - (position & (alignment - 1))) & (alignment - 1))
}

#[cfg(test)]
mod tests {
    use super::alignment_padding;
    use crate::Error;

    #[test]
    fn returns_zero_when_already_aligned() {
        assert_eq!(alignment_padding(0, 8).unwrap(), 0);
        assert_eq!(alignment_padding(8, 8).unwrap(), 0);
        assert_eq!(alignment_padding(16, 8).unwrap(), 0);
    }

    #[test]
    fn computes_padding_for_unaligned_positions() {
        assert_eq!(alignment_padding(1, 8).unwrap(), 7);
        assert_eq!(alignment_padding(5, 4).unwrap(), 3);
        assert_eq!(alignment_padding(13, 8).unwrap(), 3);
        assert_eq!(alignment_padding(15, 8).unwrap(), 1);
    }

    #[test]
    fn rejects_invalid_alignments() {
        assert!(matches!(alignment_padding(10, 0), Err(Error::InvalidAlignment)));
        assert!(matches!(alignment_padding(10, 3), Err(Error::InvalidAlignment)));
        assert!(matches!(alignment_padding(10, 6), Err(Error::InvalidAlignment)));
    }
}
