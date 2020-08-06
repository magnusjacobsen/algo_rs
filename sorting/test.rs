pub mod insertion;
pub mod selection;
use std::io::Result;

pub mod tests {
    use super::*;

    fn get_unsorted_vec() -> Vec<i32> {
        vec![1,2,3,0,-1,4,-2]
    }

    fn get_sorted_vec() -> Vec<i32> {
        vec![-2,-1,0,1,2,3,4]
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
    pub fn test_selection() -> Result<()> {
        let mut a = get_unsorted_vec();
        let b = get_sorted_vec();
        selection::sort(&mut a);
        assert_eq!(a, b);
        Ok(())    
    }
}