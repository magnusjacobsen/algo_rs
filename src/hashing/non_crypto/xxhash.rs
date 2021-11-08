use crate::util;

const PRIME_1: u32 = 0x9e3779b1; // 2654435761
const PRIME_2: u32 = 0x85ebca77; // 2246822519
const PRIME_3: u32 = 0xc2b2ae3d; // 3266489917
const PRIME_4: u32 = 0x27d4eb2f; //  668265263
const PRIME_5: u32 = 0x165667b1; //  374761393

// mixes input into acc
#[inline(always)]
fn xxh_round(mut acc: u32, input: u32) -> u32 {
    acc = input.wrapping_mul(PRIME_2).wrapping_add(acc);
    acc = util::bitwise::rotl32(acc, 13);
    acc.wrapping_mul(PRIME_1)
}

// avalanche part!!
// mixes all bits to finalize the hash
#[inline(always)]
fn xxh_avalanche(mut hash: u32) -> u32 {
    hash ^= hash >> 15;
    hash = hash.wrapping_mul(PRIME_2);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(PRIME_3);
    hash ^ (hash >> 16)
}

// based on https://github.com/easyaspi314/xxhash-clean/blob/master/xxhash32-ref.c
// not very optimized
pub fn hash(data: &[u8], length: Option<usize>, seed: u32) -> u32 {
    let mut remaining = length.unwrap_or_else(|| data.len());
    let mut offset: usize = 0;
    let mut hash: u32;

    if remaining >= 16 {
        // init accumulators
        let mut acc1 = seed.wrapping_add(PRIME_1).wrapping_add(PRIME_2);
        let mut acc2 = seed.wrapping_add(PRIME_2);
        let mut acc3 = seed;
        let mut acc4 = seed.wrapping_sub(PRIME_1);

        while remaining >= 16 {
            acc1 = xxh_round(acc1, util::types::four_bytes_to_u32(data[offset], data[offset + 1], data[offset + 2], data[offset + 3]));
            offset += 4;
            acc2 = xxh_round(acc2, util::types::four_bytes_to_u32(data[offset], data[offset + 1], data[offset + 2], data[offset + 3]));
            offset += 4;
            acc3 = xxh_round(acc3, util::types::four_bytes_to_u32(data[offset], data[offset + 1], data[offset + 2], data[offset + 3]));
            offset += 4;
            acc4 = xxh_round(acc4, util::types::four_bytes_to_u32(data[offset], data[offset + 1], data[offset + 2], data[offset + 3]));
            offset += 4;
            remaining -= 16;
        }

        hash = util::bitwise::rotl32(acc1, 1)
            .wrapping_add(util::bitwise::rotl32(acc2, 7))
            .wrapping_add(util::bitwise::rotl32(acc3, 12))
            .wrapping_add(util::bitwise::rotl32(acc4, 18));
    } else { // not enough data for the main loop to do stuff
        hash = seed.wrapping_add(PRIME_5);
    }

    hash += length.unwrap_or_else(|| data.len()) as u32;

    // now process remaining data
    while remaining >= 4 {
        let four_bytes = util::types::four_bytes_to_u32(data[offset], data[offset + 1], data[offset + 2], data[offset + 3]).wrapping_mul(PRIME_3);
        hash = four_bytes.wrapping_add(hash);
        hash = util::bitwise::rotl32(hash, 17);
        hash = hash.wrapping_mul(PRIME_4);
        offset += 4;
        remaining -= 4;
    }

    while remaining != 0 {
        let one_byte = (data[offset] as u32).wrapping_mul(PRIME_5);
        hash = one_byte.wrapping_add(hash);
        hash = util::bitwise::rotl32(hash, 11);
        hash = hash.wrapping_mul(PRIME_1);
        remaining -= 1;
        offset += 1;
    }

    xxh_avalanche(hash)
}