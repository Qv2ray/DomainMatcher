use std::num::Wrapping;

pub trait MemHash {
    fn mem_hash<T: AsRef<[u8]>>(self, bytes: T) -> u32;
}

// Constants for multiplication: four random odd 64-bit numbers.
const M1: Wrapping<u64> = Wrapping(16877499708836156737);
const M2: Wrapping<u64> = Wrapping(2820277070424839065);
const M3: Wrapping<u64> = Wrapping(9497967016996688599);
const M4: Wrapping<u64> = Wrapping(15839092249703872147);

#[inline(always)]
fn rotl31(x: Wrapping<u64>) -> Wrapping<u64> {
    return (x << 31) | (x >> (64 - 31));
}

impl MemHash for u32 {
    fn mem_hash<T: AsRef<[u8]>>(self, bytes: T) -> u32 {
        let p = bytes.as_ref();
        let mut ptr = bytes.as_ref().as_ptr();
        let mut len = bytes.as_ref().len();
        let mut h = Wrapping((self as usize + len) as u64);

        loop {
            match len {
                0 => break,
                1..=3 => {
                    h ^= Wrapping(p[0] as u64);
                    h ^= Wrapping(p[len >> 1] as u64) << 8;
                    h ^= Wrapping(p[len - 1] as u64) << 16;
                    h = rotl31(h * M1) * M2;
                    break;
                }
                4..=8 => unsafe {
                    h ^= Wrapping((ptr as *const u32).read_unaligned() as u64);
                    h ^= Wrapping(
                        (ptr.offset((len - 4) as isize) as *const u32).read_unaligned() as u64,
                    ) << 32;
                    h = rotl31(h * M1) * M2;
                    break;
                },
                9..=16 => unsafe {
                    h ^= Wrapping((ptr as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    h ^= Wrapping((ptr.offset((len - 8) as isize) as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    break;
                },
                17..=32 => unsafe {
                    h ^= Wrapping((ptr as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    h ^= Wrapping((ptr.offset(8) as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    h ^= Wrapping((ptr.offset((len - 16) as isize) as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    h ^= Wrapping((ptr.offset((len - 8) as isize) as *const u64).read_unaligned());
                    h = rotl31(h * M1) * M2;
                    break;
                },
                _ => unsafe {
                    let mut v1 = h;
                    let mut v2 = Wrapping(self as u64);
                    let mut v3 = Wrapping(self as u64);
                    let mut v4 = Wrapping(self as u64);
                    while len >= 32 {
                        v1 ^= Wrapping((ptr as *const u64).read_unaligned());
                        v1 = rotl31(v1 * M1) * M2;
                        v2 ^= Wrapping((ptr.offset(8) as *const u64).read_unaligned());
                        v2 = rotl31(v2 * M2) * M3;
                        v3 ^= Wrapping((ptr.offset(16) as *const u64).read_unaligned());
                        v3 = rotl31(v3 * M3) * M4;
                        v4 ^= Wrapping((ptr.offset(24) as *const u64).read_unaligned());
                        v4 = rotl31(v4 * M4) * M1;
                        ptr = ptr.offset(32);
                        len -= 32;
                    }
                    h = v1 ^ v2 ^ v3 ^ v4;
                },
            }
        }
        h ^= h >> 29;
        h *= M3;
        h ^= h >> 32;
        h.0 as u32
    }
}

#[test]
fn test_murmur_hash() {
    // 1..3
    assert_eq!(29u32.mem_hash(b"!"), 3384973587);
    assert_eq!(0u32.mem_hash(b"abc"), 1955219658);
    // 4..=8
    assert_eq!(20u32.mem_hash(b"adse"), 3777490067);
    assert_eq!(20u32.mem_hash(b"adsewhat"), 1791704035);
    // 9..=16
    assert_eq!(0u32.mem_hash(b"iamwhatiam.com"), 1043576267u32);
    // 16..=32
    assert_eq!(29u32.mem_hash(b"Hello, DomainMatcher!"), 1037925162);
    // >32
    assert_eq!(
        29u32.mem_hash(b"the quick brown fox jumps over the lazy dog"),
        3043016509
    );
    assert_eq!(
        0u32.mem_hash(b"adservice%5c.google%5c.%28%5ba-z%5d%7b2%7d%7ccom?)(\\.[a-z]{2})?$"),
        2880172322u32
    );
}
