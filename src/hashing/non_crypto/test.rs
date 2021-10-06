#[cfg(test)]
pub mod test {
    use crate::hashing::non_crypto::{fnv, murmur, xxhash};
    use crate::util;
    use std::io::Result;

    // just to test that it actually compiles
    #[test]
    pub fn test_xxhash_compilation() -> Result<()> {
        let a = "hello";
        let b = "my friend";
        let res1 = xxhash::hash(a, 1337);
        let res2 = xxhash::hash(b, 1337);
        assert_ne!(res1, res2);
        Ok(())
    }

    #[test]
    pub fn test_murmur_compilation() -> Result<()> {
        let a = "hello";
        let b = "my friend";
        let res1 = murmur::hash(a, 1337);
        let res2 = murmur::hash(b, 1337);
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
}