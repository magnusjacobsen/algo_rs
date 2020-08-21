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

/*
    Fisher-Yates shuffle 
    
    - based off https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle

    - if shuffle is used multiple times, with_seed (with a different seed each time) 
        or with_rng (with the same rng struct each time) should be used
    - otherwise, the content will be shuffled with the exact same sequence each time
    */
pub fn shuffle<T>(a: &mut [T]) {
    let mut rng = Mulberry32::new();
    with_rng(a, &mut rng);
}

pub fn with_seed<T>(a: &mut [T], seed: u32) {
    let mut rng = Mulberry32::with_seed(seed);
    with_rng(a, &mut rng);
}

pub fn with_rng<T>(a: &mut [T], rng: &mut Mulberry32) {
    let n = a.len();
    for i in (1..n).rev() {
        let j = rng.next_int() as usize % (i + 1);
        a.swap(i, j);
    }
}