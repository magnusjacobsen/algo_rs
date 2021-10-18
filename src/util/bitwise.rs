use std::ops::{Shl, BitAnd};
use std::cmp::PartialEq;


#[inline(always)]
pub fn rotl32(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

// generic version, perhaps I should implement something that is type-specific
#[inline(always)]
pub fn is_bit_set<T: Shl<Output = T> + From<u8> + BitAnd<Output = T> + PartialEq>(num: T, n: u8) -> bool {
    ((T::from(1) << T::from(n)) & num) != T::from(0)
}

#[inline(always)]
pub fn is_bit_set_u32(num: u32, n: u32) -> bool {
    ((1 << n) & num) != 0
}

#[inline(always)]
pub fn set_bit_u32(x: u32, n: u32) -> u32 {
    x | (1 << n)
}

