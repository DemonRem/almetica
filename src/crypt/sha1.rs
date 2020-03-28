/// Module that implements the SHA1 variant used in TERA.
///
/// TERA's SHA1 implementation is close to the original SHA1 algorithm, but with two differences: expanded values
/// aren't rotated and the output u32 words are little-endian.
use byteorder::{BigEndian, ByteOrder};

/// Structure representing the state of a SHA1 computation
/// Direct port the the JS implementation of tera-proxy to rust (MIT).
/// https://github.com/tera-toolbox/tera-network-proxy/blob/master/lib/connection/encryption/sha0.js
pub struct Sha1 {
    digest: [u32; 5],
    block: [u8; 64],
    block_index: usize,
    length: u64,
    computed: bool,
}

impl Sha1 {
    /// Construct a `Sha1` object
    pub fn new() -> Sha1 {
        let st = Sha1 {
            digest: consts::H,
            block: [0; 64],
            block_index: 0,
            length: 0,
            computed: false,
        };
        st
    }

    /// Update the hash with new data
    pub fn update(&mut self, data: &[u8]) {
        for b in data {
            self.block[self.block_index] = *b;
            self.block_index += 1;
            self.length += 8;
            if self.block_index == 64 {
                self.process_message_block();
            }
        }
    }

    /// Calculate the final hash
    pub fn hash(&mut self) -> Result<[u32; 5], std::io::Error> {
        if !self.computed {
            self.pad_message();
            self.computed = true;
        }

        Ok(self.digest)
    }

    fn process_message_block(&mut self) {
        let mut w: [u32; 80] = [0; 80];

        // Break chunk into sixteen u32 big-endian words
        for i in 0..16 {
            w[i] = BigEndian::read_u32(&self.block[i*4..]);
        }

        // Message schedule: extend the sixteen u32 into eighty u32
        for i in 16..80 {
            w[i] = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
        }

        // Initialize hash value for this chunk
        let mut a = self.digest[0];
        let mut b = self.digest[1];
        let mut c = self.digest[2];
        let mut d = self.digest[3];
        let mut e = self.digest[4];

        // Main loop
        for i in 0..80 {
            let mut temp = e.wrapping_add(left_rotate(a, 5)).wrapping_add(w[i]);
            if i < 20 {
                temp = temp.wrapping_add((b & c) | ((!b) & d));
                temp = temp.wrapping_add(consts::K[0]);
            } else if i < 40 {
                temp = temp.wrapping_add(b ^ c ^ d);
                temp = temp.wrapping_add(consts::K[1]);
            } else if i < 60 {
                temp = temp.wrapping_add((b & c) | (b & d) | (c & d));
                temp = temp.wrapping_add(consts::K[2]);
            } else {
                temp = temp.wrapping_add(b ^ c ^ d);
                temp = temp.wrapping_add(consts::K[3]);
            }
            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = temp;
        }

        // Add this chunk's hash to result so far
        self.digest[0] = self.digest[0].wrapping_add(a);
        self.digest[1] = self.digest[1].wrapping_add(b);
        self.digest[2] = self.digest[2].wrapping_add(c);
        self.digest[3] = self.digest[3].wrapping_add(d);
        self.digest[4] = self.digest[4].wrapping_add(e);

        self.block_index = 0;
    }

    fn pad_message(&mut self) {
        // Check to see if the current message block is too small to hold
        // the initial padding bits and length.  If so, we will pad the
        // block, process it, and then continue padding into a second
        // block.
        self.block[self.block_index] = 0x80;
        self.block_index += 1;

        if self.block_index > 55 {
            for i in self.block_index..64 {
                self.block[i] = 0;
                self.block_index += 1;
            }
            self.process_message_block();
        }

        if self.block_index < 56 {
            for i in self.block_index..56 {
                self.block[i] = 0;
                self.block_index += 1;
            }
        }

        BigEndian::write_u64(&mut self.block[56..], self.length);

        self.process_message_block();
    }
}

#[inline]
fn left_rotate(word: u32, shift: u32) -> u32 {
    (word << shift) | (word >> (32 - shift))
}

mod consts {
    pub const H: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    pub const K: [u32; 4] = [0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6];
}

#[cfg(test)]
mod tests {
    use super::Sha1;
    use hex::encode;
    use byteorder::{LittleEndian, ByteOrder};

    // Helper function
    fn digest_to_hex(msg: &str) -> String {
        let mut h = Sha1::new();
        h.update(&msg.as_bytes());
        let hash = h.hash().unwrap();
        let mut buf = [0; 20];
        for i in 0..5 {
            LittleEndian::write_u32(&mut buf[i*4..], hash[i])
        }
        encode(buf)
    }

    #[test]
    fn test_sha1_empty() {
        assert_eq!(
            "19ea6cf956ddd18a4a08ac1710c6923defc00877",
            digest_to_hex("")
        );
    }

    #[test]
    fn test_sha1_hello_world() {
        assert_eq!(
            "c382ce9f95c18748a2b3403b85183e88a6a84f0c",
            digest_to_hex("hello world")
        );
        assert_eq!(
            "cd4df1db2c067776df20233f305e1c8bb9101d94",
            digest_to_hex("hello, world")
        );
        assert_eq!(
            "8a3e3ab2ba039d638aa171b17a1a477b06d19b53",
            digest_to_hex("Hello, World")
        );
    }
}
