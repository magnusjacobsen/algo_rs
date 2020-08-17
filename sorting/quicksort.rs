/* 
    Quicksort
    - recursively divides and orders the list into two partitions
    --> one with no larger than v (left side)
    --> and one with no items smaller than v (right side)
    - it is randomized, because it shuffles the input

    - Running time: quadratic
    --> worst case: n^2
    --> avg case:   n log n
    --> best case:  n log n

    Despite worse worst case than Mergesort, Quicksort is usually faster:
    - it has better locality (no extra space)
    - shuffling reduces likelihood of worst case scenarions

    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 289-291    
*/

mod util;

pub fn sort<T>(a: &mut [T]) {
    let n = a.len();
    util.shuffle(a);
    rec_sort(a, 0, n);
}

fn rec_sort<T>(a: &mut [T], lo: usize, hi: usize) {
    if hi > lo {
        let j = partition(a, lo, hi);
        rec_sort(a, lo, j - 1);
        rec_sort(a, j + 1, j);
    }
}

fn partition<T>(a: &mut [T], lo: usize, hi: usize) -> usize {
    // left and right scan indexii
    let mut i = lo + 1;
    let mut j = hi;
    let v = a[lo] // partitioning item
    loop {
        while a[i] < v { // scan right
            i += 1;
            if i == hi {
                break;
            }
        }
        while v < a[j] { // scan left
            j -= 1;
            if j == lo {
                break;
            }
        }
        if i >= j { // check for complete scan
            break;
        }
        // exchange
        a.swap(i, j);
    }
    a.swap(lo, j);
    j
}