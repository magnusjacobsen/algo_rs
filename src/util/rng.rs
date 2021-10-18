/*
    Mulberry32 generator

    code inspired from:
    - https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript/47593316#47593316
    - https://gist.github.com/tommyettinger/46a874533244883189143505d203312c

    Maybe not the best random number generator, but it is:
    - sufficiently fast
    - generates numbers that are sufficiently "random" (spread out and distribution)
*/
pub struct Mulberry32 {
    a: u32
}

#[allow(dead_code)]
impl Mulberry32 {
    pub fn new() -> Self {
        Mulberry32 {a: 1337 ^ 0xDEADBEEF}
    }

    pub fn with_seed(seed: u32) -> Self {
        Mulberry32 {a: seed}
    }

    pub fn next_int(&mut self) -> u32 {
        self.a = self.a.overflowing_add(0x6D2B79F5).0;
        let mut t = self.a;
        t = (t ^ (t >> 15)).overflowing_mul(t | 1).0;
        t ^= t.overflowing_add((t ^ (t >> 7)).overflowing_mul(t | 61).0).0;
        t ^ (t >> 14)
    }
}