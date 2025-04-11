use crate::SizeDecoder;
use super::SizeEncoder;

pub struct DefaultSizeEncoder {
    encoded_length: usize,
}

impl DefaultSizeEncoder {
    pub fn new(encoded_length: usize) -> Self {
        Self { encoded_length }
    }
}

impl SizeEncoder for DefaultSizeEncoder {
    fn encode(&self, size: usize) -> Result<Vec<u8>, String> {
        const BITS_IN_BYTE: usize = 8;
        let mut encoded_size = vec![0; self.encoded_length];
        encoded_size.iter_mut().enumerate().for_each(
            |(i, c)| {
                let less_significant_byte = ((size >> (BITS_IN_BYTE * i)) & 0xFF) as u8;
                *c = less_significant_byte;
            }
        );
        Ok(encoded_size.to_vec())
    }
}

pub struct DefaultSizeDecoder {
    encoded_length: usize,
}

impl DefaultSizeDecoder {
    pub fn new(encoded_length: usize) -> Self {
        Self { encoded_length }
    }
}

impl SizeDecoder for DefaultSizeDecoder {
    fn raw_data_size(&self) -> usize {
        self.encoded_length
    }

    fn decode(&self, raw_data: &[u8]) -> Result<usize, String> {
        if raw_data.len() != self.encoded_length {
            return Err(format!("received encoded data size ({}) has unexpected length (expected {})", raw_data.len(), self.encoded_length));
        }
        const BITS_IN_BYTE: usize = 8;
        let mut decoded_size: usize = 0;
        for &byte in raw_data.iter().rev() {
            decoded_size <<= BITS_IN_BYTE;
            decoded_size |= byte as usize;
        }
        Ok(decoded_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_sanity() {
        // GIVEN
        let encoded_size_len = 4;
        let test_sizes = [u32::MIN as usize, 10, 1234, 145670, u32::MAX as usize];

        // WHEN
        let encoder = DefaultSizeEncoder::new(encoded_size_len);
        let decoder = DefaultSizeDecoder::new(encoded_size_len);

        // THEN
        test_sizes.iter().for_each(
            |s| {
                let encoded_size = encoder.encode(*s).unwrap();
                let decoded_size = decoder.decode(&encoded_size).unwrap();
                assert_eq!(*s, decoded_size);
            }
        );
    }
}
