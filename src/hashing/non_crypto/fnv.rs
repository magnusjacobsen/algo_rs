// based on wikipedia pseudo code for Fowler-Noll-Vo hash function, specifically version 1a
// FNV-1a
pub fn hash(s: &str) -> u32 {
    const OFFSET_BASIS: u32 = 0x811c9dc5;
    const PRIME: u32 = 0x1000193;
    s.as_bytes().iter().fold(OFFSET_BASIS, |h, b| (h ^ *b as u32).wrapping_mul(PRIME))
}

/*fn fnv_1(s: &str) -> u32 {
    const OFFSET_BASIS: u32 = 0x811c9dc5; 
    const PRIME: u32 = 0x1000193;
    s.as_bytes().iter().fold(OFFSET_BASIS, |h, b| h.wrapping_mul(PRIME) ^ *b as u32)
}*/