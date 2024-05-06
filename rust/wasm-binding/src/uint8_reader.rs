use std::io::{Read, Result};

pub struct Uint8ArrayReader {
    array: js_sys::Uint8Array,
    pos: usize,
}

impl Uint8ArrayReader {
    pub fn new(array: js_sys::Uint8Array) -> Self {
        Self { array, pos: 0 }
    }
}

impl Read for Uint8ArrayReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let remaining = self.array.length() as usize - self.pos;
        let to_read = std::cmp::min(buf.len(), remaining);

        if to_read == 0 {
            return Ok(0);
        }

        #[allow(clippy::indexing_slicing)]
        buf[..to_read].copy_from_slice(&self.array.to_vec()[self.pos..self.pos + to_read]);
        self.pos += to_read;

        Ok(to_read)
    }
}
