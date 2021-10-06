use crate::util;

// Murmur3, based on the pseudocode on wikipedia
pub fn hash(s: &str, seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    #[inline(always)]
    fn bytes_op(mut bytes: u32) -> u32 {
        bytes = bytes.wrapping_mul(C1);
        bytes = util::bitwise::rotl32(bytes, R1);
        bytes.wrapping_mul(C2)
    }

    let mut hash = s.as_bytes()
        .chunks(4)
        .fold(seed, |mut h, x| {
            match x {
                &[a,b,c,d]  => { h ^= bytes_op(util::types::four_bytes_to_u32(a, b, c, d));
                                 h = util::bitwise::rotl32(h, R2);
                                 h.wrapping_mul(M).wrapping_add(N) 
                               },
                &[a,b,c]    => h ^ bytes_op(util::types::three_bytes_to_u32(a, b, c)),
                &[a,b]      => h ^ bytes_op(util::types::two_bytes_to_u32(a, b)),
                &[a]        => h ^ bytes_op(a as u32),
                _           => panic!("unexpected input"),    
            }
        });
    
    hash ^= s.as_bytes().len() as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^ (hash >> 16) 
}