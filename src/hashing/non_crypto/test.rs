#[cfg(test)]
pub mod t {
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
        let res1 = xxhash::hash(a.as_bytes(), None, SEED);
        let res2 = xxhash::hash(b.as_bytes(), None, SEED);
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

    #[test]
    pub fn test_xxhash_values() -> Result<()> {
        // data preparation
        let prime: u32 = 0x9e3779b1;
        let test_data_size = 101;
        let mut test_data: Vec<u8> = vec![0; test_data_size];
        let mut byte_gen: u32 = prime;

        // generate test data
        (0..test_data_size).for_each(|i| {
            test_data[i] = (byte_gen >> 24) as u8;
            byte_gen = byte_gen.wrapping_mul(byte_gen);
        });

        assert_eq!(
            xxhash::hash(&test_data, Some(1), 0), 
            0xB85CBEE5
        );
        assert_eq!(
            xxhash::hash(&test_data, Some(1), prime), 
            0xD5845D64
        );
        assert_eq!(
            xxhash::hash(&test_data, Some(14), 0), 
            0xE5AA0AB4
        );
        assert_eq!(
            xxhash::hash(&test_data, Some(14), prime), 
            0x4481951D
        );
        assert_eq!(
            xxhash::hash(&test_data, Some(test_data_size), 0), 
            0x1F1AA412
        );
        assert_eq!(
            xxhash::hash(&test_data, Some(test_data_size), prime), 
            0x498EC8E2
        );
        Ok(())
    }

    #[bench]
    pub fn bench_xxhash_large(b: &mut Bencher) {
        let a = get_str(LARGE);
        b.iter(|| xxhash::hash(&a.as_bytes(), None, SEED));
    }
    
    #[bench]
    pub fn bench_xxhash_small(b: &mut Bencher) {
        let a = get_str(SMALL);
        b.iter(|| xxhash::hash(&a.as_bytes(), None, SEED));
    }

    #[bench]
    pub fn bench_xxhash_tiny(b: &mut Bencher) {
        let a = get_str(TINY);
        b.iter(|| xxhash::hash(&a.as_bytes(), None, SEED));
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