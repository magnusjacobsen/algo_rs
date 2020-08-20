/* 
    Mergesort
    - first the list is recursively divided down to pairs of 2
    - then the algorithm sorts those pairs, and then
    - merges all the pairs until the entire list is mergesorted

    - Running time: quadratic
    --> worst case: n log n
    --> avg case:   n log n
    --> best case:  n log n
    
    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 270-273
*/
pub fn sort<T: Ord + Default + Clone + Copy>(a: &mut [T], top_down: bool) {
    let n = a.len();
    let mut aux: Vec<T> = vec![Default::default(); n];
    if top_down {
        sort_top_down(a, &mut aux, 0, n - 1);
    } else {
        sort_bottom_up(a, &mut aux);
    }
}

fn sort_bottom_up<T: Ord + Copy>(a: &mut [T], aux: &mut [T]) {
    let n = a.len();
    let mut sz = 1;
    while sz < n {
        let mut lo = 0;
        while lo < n - sz {
            merge(a, aux, lo, lo + sz - 1, 
                std::cmp::min(lo + sz + sz - 1, n - 1));
            lo = lo + sz + sz;
        }
        sz = sz + sz;
    }
}

fn sort_top_down<T: Ord + Copy>(a: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    // first sort a[lo .. hi]
    if hi > lo {
        let mid = lo + (hi - lo) / 2;
        // sort left half
        sort_top_down(a, aux, lo, mid);
        // sort right half
        sort_top_down(a, aux, mid + 1, hi);
        // merge the two halfs
        merge(a, aux, lo, mid, hi);
    }
}

fn merge<T: Ord + Copy>(a: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    // tracks current index for left part
    let mut i = lo;
    // tracks current index for right part
    let mut j = mid + 1;

    // tracks index where to place next sorted item
    let mut k = lo;
    // copy a[a .. hi] to aux[lo ... hi]
    while k <= hi {
        aux[k] = a[k];
        k += 1;
    }

    // merge back to a[lo .. hi]
    k = lo;
    while k <= hi {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j > hi {
            a[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            a[k] = aux[j];
            j += 1;
        } else {
            a[k] = aux[i];
            i += 1;
        }
        k += 1;
    }
}