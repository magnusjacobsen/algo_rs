#[cfg(test)]
pub mod test {
    use crate::util;
    use std::io::Result;

    #[test]
    pub fn binary_search_index() -> Result<()> {
        let a: Vec<usize> = (0..200).collect();
        let res = util::binary_search::get_index(100, &a).unwrap();
        assert_eq!(res, 100);
        Ok(())
    }

    #[test]
    pub fn binary_search_index_fail() -> Result<()> {
        let a: Vec<i32> = (0..200).collect();
        let res = util::binary_search::get_index(201, &a);
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    pub fn binary_search_index_negative_numbers() -> Result<()> {
        let n = 100000;
        let a: Vec<i32> = (0..n).map(|x| -x).rev().collect();
        let res = util::binary_search::get_index(0, &a).unwrap();
        assert_eq!(res, n as usize - 1);
        Ok(())
    }
}