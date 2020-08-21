#[cfg(test)]
pub mod test {
    use crate::sorting::{insertion, mergesort, selection, shellsort, radix, quicksort};
    use crate::util;
    use std::io::Result;

    fn get_unsorted_vec() -> Vec<i32> {
        vec![1,2,3,0,-1,4,-2]
    }

    fn get_sorted_vec() -> Vec<i32> {
        vec![-2,-1,0,1,2,3,4]
    }

    fn get_sorted_vec_large() -> Vec<i32> {
        let n = 10_000;
        (0..n).map(|x| x / 2).collect()
    }

    fn get_unsorted_vec_large() -> Vec<i32> {
        let n = 10_000;
        let mut v: Vec<i32> = (0..n).map(|x| x / 2).collect();
        util::shuffle(&mut v);
        v
    }

    #[test]
    pub fn test_setup() -> Result<()> {
        let a = get_unsorted_vec();
        let b = get_sorted_vec();
        assert_ne!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_setup_2() -> Result<()> {
        let a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        assert_ne!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_insertion() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        insertion::sort(&mut a);
        assert_eq!(a,b);
        Ok(())
    }

    #[test]
    pub fn test_insertion_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        insertion::sort(&mut a);
        assert_eq!(a,b);
        Ok(())
    }

    #[test]
    pub fn test_selection() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        selection::sort(&mut a);
        assert_eq!(a, b);
        Ok(())    
    }

    #[test]
    pub fn test_selection_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        selection::sort(&mut a);
        assert_eq!(a, b);
        Ok(())    
    }

    #[test]
    pub fn test_merge_top_down() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        mergesort::sort(&mut a, true);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_merge_top_down_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        mergesort::sort(&mut a, true);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_merge_bottom_up() -> Result<()> {
        let mut a = get_unsorted_vec();
        println!("og: {:?}", a);
        let b = get_sorted_vec();
        mergesort::sort(&mut a, false);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_merge_bottom_up_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        mergesort::sort(&mut a, false);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_shellsort() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        shellsort::sort(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_shellsort_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        shellsort::sort(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_signed_i32() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        radix::sort_signed(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_signed_i32_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        radix::sort_signed(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_signed_i16() -> Result<()> {
        let mut a: Vec<i16> = get_unsorted_vec()
            .iter()
            .map(|x| *x as i16)
            .collect();
        let b: Vec<i16> = get_sorted_vec()
            .iter()
            .map(|x| *x as i16)
            .collect();
        radix::sort_signed(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_signed_i64() -> Result<()> {
        let mut a: Vec<i64> = get_unsorted_vec()
            .iter()
            .map(|x| *x as i64)
            .collect();
        let b: Vec<i64> = get_sorted_vec()
            .iter()
            .map(|x| *x as i64)
            .collect();
        radix::sort_signed(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_unsigned_u32() -> Result<()> {
        let mut a: Vec<u32> = vec![5,4,2,3,1];
        let b: Vec<u32> = vec![1,2,3,4,5];
        radix::sort_unsigned(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_unsigned_u16() -> Result<()> {
        let mut a: Vec<u16> = vec![5,4,2,3,1];
        let b: Vec<u16> = vec![1,2,3,4,5];
        radix::sort_unsigned(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_radix_unsigned_u64() -> Result<()> {
        let mut a: Vec<u64> = vec![u64::MAX,5,4,2,1,0,3,1];
        let b: Vec<u64> = vec![0,1,1,2,3,4,5,u64::MAX];
        radix::sort_unsigned(&mut a);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_quicksort() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        quicksort::sort(&mut a, false);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_quicksort_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        quicksort::sort(&mut a, false);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_quick_3_way() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        quicksort::sort(&mut a, true);
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    pub fn test_quick_3_way_large() -> Result<()> {
        let mut a = get_unsorted_vec_large();
        let b = get_sorted_vec_large();
        quicksort::sort(&mut a, true);
        assert_eq!(a, b);
        Ok(())
    }
}