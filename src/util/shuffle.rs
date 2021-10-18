use crate::util::rng;
/*
    Fisher-Yates shuffle 
    
    - based off https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle

    - if shuffle is used multiple times, with_seed (with a different seed each time) 
        or with_rng (with the same rng struct each time) should be used
    - otherwise, the content will be shuffled with the exact same sequence each time
    */
pub fn shuffle<T>(a: &mut [T]) {
    let mut rng = rng::Mulberry32::new();
    with_rng(a, &mut rng);
}

pub fn with_seed<T>(a: &mut [T], seed: u32) {
    let mut rng = rng::Mulberry32::with_seed(seed);
    with_rng(a, &mut rng);
}

pub fn with_rng<T>(a: &mut [T], rng: &mut rng::Mulberry32) {
    let n = a.len();
    for i in (1..n).rev() {
        let j = rng.next_int() as usize % (i + 1);
        a.swap(i, j);
    }
}