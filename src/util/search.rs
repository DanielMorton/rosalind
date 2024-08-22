use std::fmt::Debug;

pub(crate) fn binary_search<T: PartialOrd>(arr: &[T], val: T) -> i32 {
    let (mut left, mut right) = (0, arr.len());
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == val {
            return (mid + 1) as i32;
        } else if arr[mid] < val {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}
