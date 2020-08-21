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
    let mut aux: Vec<T> = vec![Default::default(); a.len()];
    if top_down {
        sort_top_down(a, &mut aux);
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
            let hi = std::cmp::min(lo + sz + sz, n);
            merge(&mut a[lo..hi], &mut aux[lo..hi], sz - 1);
            lo = lo + sz + sz;
        }
        sz = sz + sz;
    }
}

fn sort_top_down<T: Ord + Copy>(a: &mut [T], aux: &mut [T]) {
    let n = a.len() - 1;
    // first sort a[lo .. hi]
    if n > 0 {
        let mid = n / 2;
        // sort left half
        sort_top_down(&mut a[..mid + 1], &mut aux[..mid + 1]);
        // sort right half
        sort_top_down(&mut a[mid + 1..], &mut aux[mid + 1..]);
        // merge the two halfs

        merge(a, aux, mid);
    }
}

fn merge<T: Ord + Copy>(a: &mut [T], aux: &mut [T], mid: usize) {
    let hi = a.len();
    // tracks current index for left part
    let mut i = 0;
    // tracks current index for right part
    let mut j = mid + 1;

    // tracks index where to place next sorted item
    let mut k = 0;
    // copy a[a .. hi] to aux[lo ... hi]
    while k < hi {
        aux[k] = a[k];
        k += 1;
    }

    // merge back to a[lo .. hi]
    k = 0;
    while k < hi {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j >= hi {
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