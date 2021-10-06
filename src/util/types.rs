// little endian
#[inline(always)]
pub fn four_bytes_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
}

// little endian
#[inline(always)]
pub fn three_bytes_to_u32(a: u8, b: u8, c: u8) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16)
}

// little endian
#[inline(always)]
pub fn two_bytes_to_u32(a: u8, b: u8) -> u32 {
    (a as u32) | ((b as u32) << 8)
}