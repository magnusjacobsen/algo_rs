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
use crate::util;

pub fn sort<T: PartialOrd + Clone>(a: &mut [T], use_3_way: bool) {
    let n = a.len();
    util::shuffle(a);
    if use_3_way {
        quick_3_way(a, 0, n - 1);
    } else {
        rec_sort(a, 0, n - 1);
    }
}

fn rec_sort<T: PartialOrd + Clone>(a: &mut [T], lo: usize, hi: usize) {
    if hi > lo {
        let j = partition(a, lo, hi);
        if j > 0 {
            rec_sort(a, lo, j - 1);
        }
        if j < hi {
            rec_sort(a, j + 1, hi);
        }
    }
}

/*
    A version of Quicksort that's very fast on numbers with low Shannon entropy (amount of numbers with same value)

        Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 299    
    */
fn quick_3_way<T: PartialOrd + Clone>(a: &mut [T], lo: usize, hi: usize) {
    if hi > lo {
        let mut lt = lo;
        let mut i = lo + 1;
        let mut gt = hi;
        let v = a[lo].clone();// partitioning item
        while i <= gt {
            if a[i] < v { 
                a.swap(lt, i);
                i += 1;
                lt += 1;
            } else if a[i] > v {
                a.swap(i, gt);
                gt -= 1;
            } else {
                i += 1;
            } 
        } // now a[lo..lt-1] < v; a[lt..gt] == v; a[gt+1..hi] > v
        quick_3_way(a, lo, lt - 1);
        quick_3_way(a, gt + 1, hi);
    }
}

fn partition<T: PartialOrd + Clone>(a: &mut [T], lo: usize, hi: usize) -> usize {
    // left and right scan indexii
    let mut i = lo + 1;
    let mut j = hi;
    let v = a[lo].clone(); // partitioning item
    loop {
        while a[i] < v { // scan right
            i += 1;
            if i >= hi {
                break;
            }
        }
        while v < a[j] { // scan left
            j -= 1;
            if j <= lo {
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