struct Mulberry32 {
    a: u32
}

// https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript/47593316#47593316
// https://gist.github.com/tommyettinger/46a874533244883189143505d203312c
impl Mulberry32 {
    pub fn new() -> Self {
        let seed = 1337 ^ 0xDEADBEEF;
        Mulberry32 {a: seed}
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

// Fisher-Yates shuffle - https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
pub fn shuffle<T>(a: &mut [T]) {
    let mut rng = Mulberry32::new();
    let n = a.len();
    for i in (1..n).rev() {
        let j = rng.next_int() as usize % (i + 1);
        a.swap(i, j);
    }
}