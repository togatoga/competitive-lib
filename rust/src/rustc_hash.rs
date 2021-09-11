use cargo_snippet::snippet;
#[snippet]
pub mod rustc_hash {
    /// Permission is hereby granted, free of charge, to any
    /// person obtaining a copy of this software and associated
    /// documentation files (the "Software"), to deal in the
    /// Software without restriction, including without
    /// limitation the rights to use, copy, modify, merge,
    /// publish, distribute, sublicense, and/or sell copies of
    /// the Software, and to permit persons to whom the Software
    /// is furnished to do so, subject to the following
    /// conditions:

    /// The above copyright notice and this permission notice
    /// shall be included in all copies or substantial portions
    /// of the Software.

    /// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    /// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    /// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    /// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    /// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    /// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    /// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    /// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    /// DEALINGS IN THE SOFTWARE.
    use core::convert::TryInto;
    use core::default::Default;
    use core::hash::BuildHasherDefault;
    use core::hash::Hasher;
    use core::mem::size_of;
    use core::ops::BitXor;
    use std::collections::{HashMap, HashSet};

    /// Type alias for a hashmap using the `fx` hash algorithm.
    pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

    /// Type alias for a hashmap using the `fx` hash algorithm.
    pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;

    /// A speedy hash algorithm for use within rustc. The hashmap in liballoc
    /// by default uses SipHash which isn't quite as speedy as we want. In the
    /// compiler we're not really worried about DOS attempts, so we use a fast
    /// non-cryptographic hash.
    ///
    /// This is the same as the algorithm used by Firefox -- which is a homespun
    /// one not based on any widely-known algorithm -- though modified to produce
    /// 64-bit hash values instead of 32-bit hash values. It consistently
    /// out-performs an FNV-based hash within rustc itself -- the collision rate is
    /// similar or slightly worse than FNV, but the speed of the hash function
    /// itself is much higher because it works on up to 8 bytes at a time.
    pub struct FxHasher {
        hash: usize,
    }

    #[cfg(target_pointer_width = "32")]
    const K: usize = 0x9e3779b9;
    #[cfg(target_pointer_width = "64")]
    const K: usize = 0x517cc1b727220a95;

    impl Default for FxHasher {
        #[inline]
        fn default() -> FxHasher {
            FxHasher { hash: 0 }
        }
    }

    impl FxHasher {
        #[inline]
        fn add_to_hash(&mut self, i: usize) {
            self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
        }
    }

    impl Hasher for FxHasher {
        #[inline]
        fn write(&mut self, mut bytes: &[u8]) {
            #[cfg(target_pointer_width = "32")]
            let read_usize = |bytes: &[u8]| u32::from_ne_bytes(bytes[..4].try_into().unwrap());
            #[cfg(target_pointer_width = "64")]
            let read_usize = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

            let mut hash = FxHasher { hash: self.hash };
            assert!(size_of::<usize>() <= 8);
            while bytes.len() >= size_of::<usize>() {
                hash.add_to_hash(read_usize(bytes) as usize);
                bytes = &bytes[size_of::<usize>()..];
            }
            if (size_of::<usize>() > 4) && (bytes.len() >= 4) {
                hash.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as usize);
                bytes = &bytes[4..];
            }
            if (size_of::<usize>() > 2) && bytes.len() >= 2 {
                hash.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as usize);
                bytes = &bytes[2..];
            }
            if (size_of::<usize>() > 1) && !bytes.is_empty() {
                hash.add_to_hash(bytes[0] as usize);
            }
            self.hash = hash.hash;
        }

        #[inline]
        fn write_u8(&mut self, i: u8) {
            self.add_to_hash(i as usize);
        }

        #[inline]
        fn write_u16(&mut self, i: u16) {
            self.add_to_hash(i as usize);
        }

        #[inline]
        fn write_u32(&mut self, i: u32) {
            self.add_to_hash(i as usize);
        }

        #[cfg(target_pointer_width = "32")]
        #[inline]
        fn write_u64(&mut self, i: u64) {
            self.add_to_hash(i as usize);
            self.add_to_hash((i >> 32) as usize);
        }

        #[cfg(target_pointer_width = "64")]
        #[inline]
        fn write_u64(&mut self, i: u64) {
            self.add_to_hash(i as usize);
        }

        #[inline]
        fn write_usize(&mut self, i: usize) {
            self.add_to_hash(i);
        }

        #[inline]
        fn finish(&self) -> u64 {
            self.hash as u64
        }
    }
}
