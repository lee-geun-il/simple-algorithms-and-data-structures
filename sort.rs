#![allow(dead_code)]
// BUBBLE SORT
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for index_i in 0..arr.len() {
        for index_j in 0..arr.len() - 1 - index_i {
            if arr[index_j] > arr[index_j + 1] {
                arr.swap(index_j, index_j + 1);
            }
        }
    }
}

// INSERTION SORT
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for index_i in 1..arr.len() {
        let mut index_j = index_i;
        while index_j > 0 && arr[index_j - 1] > arr[index_j] {
            arr.swap(index_j, index_j - 1);
            index_j = index_j - 1;
        }
    }
}

// SELECTION SORT
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for index_i in 0..arr.len() {
        let mut index_smallest = index_i;
        for index_j in (index_i + 1)..arr.len() {
            if arr[index_j] < arr[index_smallest] {
                index_smallest = index_j;
            }
        }
        arr.swap(index_i, index_smallest);
    }
}

// QUICK SORT
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    _quick_sort(arr, 0, (arr.len() - 1) as isize);
}
fn _quick_sort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left < right {
        let pivot = _partition(arr, left, right);
        _quick_sort(arr, left, pivot - 1);
        _quick_sort(arr, pivot + 1, right);
    }
}
fn _partition<T: Ord>(arr: &mut [T], index_lowest: isize, index_highest: isize) -> isize {
    let index_pivot = index_lowest as usize;
    let mut index_low = index_lowest;
    let mut index_high = index_highest + 1;
    loop {
        index_low += 1;
        while index_low < index_highest && arr[index_pivot] > arr[index_low as usize] {
            index_low += 1;
        }
        index_high -= 1;
        while index_high > index_lowest && arr[index_pivot] < arr[index_high as usize] {
            index_high -= 1;
        }           
        if index_low < index_high {
            arr.swap(index_low as usize, index_high as usize);
        } else {
            arr.swap(index_pivot as usize, index_high as usize);
            break;
        }
    }
    return index_high;
}

// STOOGE SORT
pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    let length = arr.len();
    if arr.first() > arr.last() {
        arr.swap(0, length - 1);
    }
    if length > 2 {
        let t = arr.len() / 3;
        stooge_sort(&mut arr[..length - t]);
        stooge_sort(&mut arr[t..]);
        stooge_sort(&mut arr[..length - t]);
    }
}

// MERGE SORT
pub fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let length = x.len();
    if length < 2 {
        return;
    }
    merge_sort(&mut x[0..length/2]);
    merge_sort(&mut x[length/2..length]);
    let mut y: Vec<T> = x.to_vec();
    _merge(&x[0..length/2], &x[length/2..length], &mut y[..]);
    x.copy_from_slice(&y);
}
fn _merge<T: Copy + Ord> (x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut index_i = 0;
    let mut index_j = 0;
    let mut index_k = 0;
    while index_i < x1.len() && index_j < x2.len() {
        if x1[index_i] < x2[index_j] {
            y[index_k] = x1[index_i];
            index_k += 1;
            index_i += 1;
        } else {
            y[index_k] = x2[index_j];
            index_k += 1;
            index_j += 1;
        }
    }
    if index_i < x1.len() {
        y[index_k..].copy_from_slice(&x1[index_i..]);
    }
    if index_j < x2.len() {
        y[index_k..].copy_from_slice(&x2[index_j..]);
    }
}
