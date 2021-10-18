#[cfg(test)]
pub mod tests {
    use crate::hashing::non_crypto::{fnv, murmur, xxhash};
    use std::io::Result;
    extern crate test;
    use test::Bencher;

    const LARGE: usize = 100_000;
    const SMALL: usize = 100;
    const TINY: usize = 7;

    const SEED: u32 = 1337;

    fn get_str(n: usize) -> String {
        vec!['a'; n].iter().collect::<String>()
    }

    // just to test that it actually compiles
    #[test]
    pub fn test_xxhash_compilation() -> Result<()> {
        let a = "hello";
        let b = "my friend";
        let res1 = xxhash::hash(a, SEED);
        let res2 = xxhash::hash(b, SEED);
        assert_ne!(res1, res2);
        Ok(())
    }

    #[test]
    pub fn test_murmur_compilation() -> Result<()> {
        let a = "hello";
        let b = "my friend";
        let res1 = murmur::hash(a, SEED);
        let res2 = murmur::hash(b, SEED);
        assert_ne!(res1, res2);
        Ok(())
    }

    #[test]
    pub fn test_fnv_compilation() -> Result<()> {
        let a = "hello";
        let b = "my friend";
        let res1 = fnv::hash(a);
        let res2 = fnv::hash(b);
        assert_ne!(res1, res2);
        Ok(())
    }

    #[bench]
    pub fn bench_xxhash_large(b: &mut Bencher) {
        let a = get_str(LARGE);
        b.iter(|| xxhash::hash(&a, SEED));
    }
    
    #[bench]
    pub fn bench_xxhash_small(b: &mut Bencher) {
        let a = get_str(SMALL);
        b.iter(|| xxhash::hash(&a, SEED));
    }

    #[bench]
    pub fn bench_xxhash_tiny(b: &mut Bencher) {
        let a = get_str(TINY);
        b.iter(|| xxhash::hash(&a, SEED));
    }

    #[bench]
    pub fn bench_murmur_large(b: &mut Bencher) {
        let a = get_str(LARGE);
        b.iter(|| murmur::hash(&a, SEED));
    }
    
    #[bench]
    pub fn bench_murmur_small(b: &mut Bencher) {
        let a = get_str(SMALL);
        b.iter(|| murmur::hash(&a, SEED));
    }

    #[bench]
    pub fn bench_murmur_tiny(b: &mut Bencher) {
        let a = get_str(TINY);
        b.iter(|| murmur::hash(&a, SEED));
    }

    #[bench]
    pub fn bench_fnv_large(b: &mut Bencher) {
        let a = get_str(LARGE);
        b.iter(|| fnv::hash(&a));
    }
    
    #[bench]
    pub fn bench_fnv_small(b: &mut Bencher) {
        let a = get_str(SMALL);
        b.iter(||  fnv::hash(&a));
    }

    #[bench]
    pub fn bench_fnv_tiny(b: &mut Bencher) {
        let a = get_str(TINY);
        b.iter(||  fnv::hash(&a));
    }
}