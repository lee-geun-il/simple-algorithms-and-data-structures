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
    index_high
}
