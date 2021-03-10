use std::num::Wrapping;

pub trait Murmur3 {
    fn murmur_hash<T: AsRef<[u8]>>(self, bytes: T) -> u32;
}

impl Murmur3 for u32 {
    fn murmur_hash<T: AsRef<[u8]>>(self, bytes: T) -> u32 {
        let ptr = bytes.as_ref().as_ptr();
        let len = bytes.as_ref().len();
        let mut h = Wrapping(self);
        let c1 = Wrapping(0xcc9e2d51u32);
        let c2 = Wrapping(0x1b873593u32);

        unsafe {
            let (block, left): (&[u32], &[u8]) = {
                let (us_len, ts_len) = (len / 4, len & 3);
                (
                    std::slice::from_raw_parts(ptr as *const u32, us_len),
                    std::slice::from_raw_parts(ptr.add(len - ts_len), ts_len),
                )
            };
            for &k in block.iter() {
                let mut k = Wrapping(k) * c1;
                k = (k << 15) | (k >> 17);
                k *= c2;
                h ^= k;
                h = (h << 13) | (h >> 19);
                h = h * Wrapping(5) + Wrapping(0xe6546b64);
            }
            let mut k = Wrapping(0u32);
            match left.len() {
                3 => {
                    k ^= Wrapping(left[2] as u32) << 16;
                    k ^= Wrapping(left[1] as u32) << 8;
                    k ^= Wrapping(left[0] as u32);
                    k *= c1;
                    k = (k << 15) | (k >> 17);
                    k *= c2;
                    h ^= k;
                }
                2 => {
                    k ^= Wrapping(left[1] as u32) << 8;
                    k ^= Wrapping(left[0] as u32);
                    k *= c1;
                    k = (k << 15) | (k >> 17);
                    k *= c2;
                    h ^= k;
                }
                1 => {
                    k ^= Wrapping(left[0] as u32);
                    k *= c1;
                    k = (k << 15) | (k >> 17);
                    k *= c2;
                    h ^= k;
                }
                0 => {}
                _ => core::hint::unreachable_unchecked(),
            }
            h ^= Wrapping(len as u32);
            h ^= h >> 16;
            h *= Wrapping(0x85ebca6b);
            h ^= h >> 13;
            h *= Wrapping(0xc2b2ae35);
            h ^= h >> 16;
        }
        h.0
    }
}

#[test]
fn test_murmur_hash() {
    assert_eq!(0u32.murmur_hash(b""), 0);
    assert_eq!(1u32.murmur_hash(b""), 0x514e28b7);
    assert_eq!(0xffffffffu32.murmur_hash(b""), 0x81f16f39);
    assert_eq!(0x9747b28cu32.murmur_hash(b"Hello, world!"), 0x24884cba);
    assert_eq!(0u32.murmur_hash(b"\xff\xff\xff\xff"), 0x76293b50);
    assert_eq!(0u32.murmur_hash(b"!Ce\x87"), 0xf55b516b);
    assert_eq!(0u32.murmur_hash(b"!Ce"), 0x7e4a8634);
    assert_eq!(0u32.murmur_hash(b"!C"), 0xa0f7b07a);
    assert_eq!(0u32.murmur_hash(b"!"), 0x72661cf4);
    assert_eq!(0u32.murmur_hash("\x00\x00\x00\x00"), 0x2362f9de);
    assert_eq!(0u32.murmur_hash("\x00\x00\x00"), 0x85f0b427);
    assert_eq!(0u32.murmur_hash("\x00\x00"), 0x30f4c306);
}
