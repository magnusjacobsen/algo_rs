/*
    Insertion sort
    - is sensitive to input

    Compares all elements to the one left of them, and if smaller
    swaps them, until the left is smaller/lesser.

    - Running time: quadratic
    --> worst case: N^2/2 compares, N^2/2 swaps
    --> avg case: N^2/4 compares, N^2/4 swaps
    --> best case: N - 1 compares, 0 swaps
    
    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 251

    -- always sorts in-place
*/
use std::cmp::Ord;

pub fn sort<T: Ord>(a: &mut [T]) {
    let n = a.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}