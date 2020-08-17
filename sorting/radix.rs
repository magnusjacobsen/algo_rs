/*
    Radix sort
    - starting with the least significant digit, the numbers are sorted in two buckets
    --> the original vector and an auxiliary vector, and then combined in the new order
    
    Running time: linear
    - all cases: n * number of bits

    Space requirement: 2 * n

    Number of compares: 0

inspired by:
https://rosettacode.org/wiki/Sorting_algorithms/Radix_sort#C.23
*/

use std::ops::{Shl, Sub, Mul};
use std::fmt::Debug;

pub fn sort_signed<T: Ord + Debug + Copy + Default + From<i8> + Shl<Output = T> + Sub<Output = T> + Mul<Output = T>>(a: &mut [T]) {
    let n = a.len();
    let mut shift = T::from(std::mem::size_of::<T>() as i8) * T::from(8) - T::from(1);
    let mut aux: Vec<T> = vec![Default::default(); n];
    while shift > T::from(-1) {
        let mut j = 0;
        for i in 0..n {
            // if the current bit is larger or equal to zero, we move
            // it to aux, and then later back to the back of a
            let to_aux = (a[i] << shift) >= T::from(0);
            // the single MSD of signed integer is negative, not positive
            let to_a = if shift == T::from(0) {
                !to_aux
            } else { 
                to_aux 
            };

            if to_a {
                a[i - j] = a[i];
            } else {
                aux[j] = a[i];
                j += 1;
            }
        }

        // copy the larger bit numbers to the back of a
        let dest_index = n - j;
        for i in 0..j {
            a[dest_index + i] = aux[i];
        }
        shift = shift - T::from(1);
    }
}

pub fn sort_unsigned<T: Ord + std::fmt::Display + Debug + Copy + Default + From<u8> + Shl<Output = T> + Sub<Output = T> + Mul<Output = T>>(a: &mut [T]) {
    let n = a.len();
    let mut shift = T::from(std::mem::size_of::<T>() as u8) * T::from(8) - T::from(1);
    let split = T::from(1) << shift;
    let mut aux: Vec<T> = vec![Default::default(); n];
    loop {
        let mut j = 0;
        for i in 0..n {
            // to a, else to aux
            if (a[i] << shift) < split {
                a[i - j] = a[i];
            } else {
                aux[j] = a[i];
                j += 1;
            }
        }
        let dest_index = n - j;
        for i in 0..j {
            a[dest_index + i] = aux[i];
        }
        if shift == T::from(0) {
            break;
        } else {
            shift = shift - T::from(1);
        }
    }
}