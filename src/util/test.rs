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

    #[test]
    /// checking that the shuffle doesn't bias too much certain positions
    /// the cutoff is no more than 1/3 of the average in either way
    fn shuffle_distribution_small_sample() -> Result<()> {
        let mut rng = util::shuffle::Mulberry32::new();
        let n = 10;
        let samples = 1000;
        let mut cnt = vec![vec![0;n];n];
        for _ in 0..samples {
            let mut a: Vec<usize> = (0..n).collect();
            util::shuffle::with_rng(&mut a, &mut rng);
            for i in 0..n {
                cnt[i][a[i]] += 1;
            }
        }
        let avg =(samples / n) as i32;
        let mut max_over = 0;
        let mut max_under = 0;
        for i in 0..n {
            for j in 0..n {
                let diff = avg - cnt[i][j] as i32;
                if diff > max_over {
                    max_over = diff;
                }
                if diff < max_under {
                    max_under = diff;
                }
            }
        }
        let allowed_diff = avg / 3;
        assert!(max_under >= -allowed_diff && max_over <= allowed_diff);
        Ok(())
    }
}