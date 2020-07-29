// MERGE SORT [GREAT]
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

// QUICK SORT [GREAT]
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

// INSERTION SORT [NOT BAD]
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for index_i in 1..arr.len() {
        let mut index_j = index_i;
        while index_j > 0 && arr[index_j - 1] > arr[index_j] {
            arr.swap(index_j, index_j - 1);
            index_j = index_j - 1;
        }
    }
}

// SELECTION SORT [NOT BAD]
pub fn selection_sort<T: Ord>(array: &mut [T]) {
    for index_i in 0..array.len() {
        let mut index_smallest = index_i;
        for index_j in (index_i + 1)..array.len() {
            if array[index_j] < array[index_smallest] {
                index_smallest = index_j;
            }
        }
        array.swap(index_i, index_smallest);
    }
}

// BUBBLE SORT [BAD]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for index_i in 0..arr.len() {
        for index_j in 0..arr.len() - 1 - index_i {
            if arr[index_j] > arr[index_j + 1] {
                arr.swap(index_j, index_j + 1);
            }
        }
    }
}

// GNOME SORT [BAD]
pub fn gnome_sort<T: Ord>(array: &mut [T]) {
    let mut index_i = 0;
    let length = array.len() as i32;
    while index_i < length - 1 {
        if array[index_i as usize] > array[(index_i + 1) as usize] {
            array.swap(index_i as usize, (index_i + 1) as usize);
            if index_i != 0 {
                index_i -= 1;
            }
        } else {
            index_i += 1;
        }
    }
}

// SLOW SORT [BAD]
pub fn slow_sort<T: Ord>(array: &mut [T]) {
    _slow_sort(array, 0, (array.len() - 1) as isize);
}
fn _slow_sort<T: Ord>(array: &mut [T], index_i: isize, index_j: isize) {
    if index_i < index_j {
        let index_middle = (index_i + index_j) / 2;
        _slow_sort(array, index_i, index_middle);
        _slow_sort(array, index_middle + 1, index_j);
        if array[index_middle as usize] > array[index_j as usize] {
            array.swap(index_middle as usize, index_j as usize);
        }
        _slow_sort(array, index_i, index_j - 1)
    }
}

// STOOGE SORT [BAD]
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
