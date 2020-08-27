/*
    Binary Search

    - it requires the input to be sorted beforehand
    
    - starts in the middle
    - checks if the key is lt, eq, or gt the mid item
    - then either:
    --> returns mid (eq)
    --> sets new mid as the mid of left half (lt)
    --> sets new mid as the mid of right half (gt)

    -- if there are duplicates of matching keys, only one will be found

    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 47
*/

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

pub fn key_is_present<T: Eq + PartialOrd>(key: T, a: &[T]) -> bool {
    get_index(key, a).is_some()
}

/*
    Exponential search

    not sure if there is an indexable stream-ish collection type for rust, 
    so I'm just using a Vec, where I could obviously get the length as a constant operation...
*/
pub fn exponential_search<T: Eq + PartialOrd>(key: T, a: &[T]) -> Option<usize> {
    let mut i = 0;
    loop {
        if a[i] == key {
            return Some(i);
        } else if a[i] < key {
            return get_index(key, &a[..i + 1]);
        } else {
            i = if i == 0 { 1 } else { i * 2 };
        }
    }
}