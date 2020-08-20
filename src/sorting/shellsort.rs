/*
    Shellsort
    - extension to insertion sort
    ---> allows swaps of entries far away from eachother, 
    ---> thus gaining speed when entries should be placed 
    ---> far away from their orignal position

    - it rearranges the list such that every h'th entry yields
    - a sorted subsequence (h-sorted)

    Running time: quadratic
    - worst case: n^2
    - best case: n log(n)
    - avg case: depends on gap sequence (h)
*/
pub fn sort<T: Ord>(a: &mut [T]) {
    let n = a.len();
    let mut h = 1;
    while h < (n / 3) {
        h = 3 * h + 1 // 1, 4, 13, 40, 121, 364, 1093 ...
    }

    while h >= 1 {
        // h-sort the list
        for i in h..n {
            let mut j = i;
            while j >= h && a[j] < a[j - h] {
                a.swap(j, j - h);
                j = j - h;
            }
        }
        h = h / 3;
    }
}