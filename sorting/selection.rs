/*
    Selection Sort
    - Pretty simple, and pretty bad

    Starts from the beginning and then searches for the next minimum
    value n times, swaps, and then the collection is sorted.

    - Running time: quadratic (hint: the nested for loop)
    --> actually it is N^2 / 2 compares, and N swaps
    --> running time is agnostic to the input

    Adapted from Algorithms, 4th edition, by Robert Sedgewick & Kevin Wayne
    --> page 249

    Sorts in-place
*/
pub fn sort<T: Ord>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let mut min = i; // index of the minimum value not yet sorted
        for j in i + 1..n {
            if a[j] < a[min] {
                min = j;    
            }
        }
        a.swap(i, min);
    }
}