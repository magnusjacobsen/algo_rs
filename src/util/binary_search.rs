/*
    Binary Search

    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 47
*/

// requires that the input collection is sorted
pub fn get_index<T: Eq + PartialOrd>(key: T, a: &[T]) -> Option<usize> {
    if a.len() == 0 {
        return None;
    }
    let mut lo = 0;
    let mut hi = a.len() - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if key < a[mid] {
            hi = mid - 1;
        } else if key > a[mid] {
            lo = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

pub fn is_present<T: Eq + PartialOrd>(key: T, a: &[T]) -> bool {
    get_index(key, a).is_some()
}