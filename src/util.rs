//! Library utilities.

/// Encodes the size of a small section of bytes. This should only be used for
/// values known to be less than 256 bytes.
#[inline]
pub const fn encode_len_small(len: usize) -> u8 {
    len as u8
}

/// Decodes the size of a small section of bytes. This should only be used for
/// values known to be less than 256 bytes.
#[inline]
pub const fn decode_len_small(len_encoded: u8) -> usize {
    len_encoded as usize
}

/// Encodes the size of a large section of bytes. This can be used for values of
/// any size.
pub fn encode_len_large(mut len: usize) -> Vec<u8> {
    let mut len_encoded = Vec::new();

    while len > 0 {
        len_encoded.push(len as u8);
        len >>= 8;
    }

    len_encoded.push(len_encoded.len() as u8);
    len_encoded.reverse();
    len_encoded
}

/// Decodes the size of a large section of bytes. This can be used for values of
/// any size.
pub fn decode_len_large(len_encoded: &[u8]) -> usize {
    let mut len = 0;

    #[allow(clippy::needless_range_loop)]
    for i in 0..len_encoded.len() {
        len = (len << 8) + (len_encoded[i] as usize);
    }

    len
}
