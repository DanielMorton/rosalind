pub(crate) fn inversion_count<T: PartialOrd + Copy>(arr: &[T]) -> usize {
    arr.iter()
        .enumerate()
        .map(|(i, n)| arr[i + 1..].iter().filter(|&a| a < n).count())
        .sum()
}

pub(crate) fn merge<T: PartialOrd + Copy>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut arr = Vec::new();
    let (mut i1, mut i2) = (0, 0);
    while i1 < arr1.len() && i2 < arr2.len() {
        if arr1[i1] < arr2[i2] {
            arr.push(arr1[i1]);
            i1 += 1
        } else {
            arr.push(arr2[i2]);
            i2 += 1
        }
    }
    if i1 == arr1.len() {
        arr.extend_from_slice(&arr2[i2..]);
    } else if i2 == arr2.len() {
        arr.extend_from_slice(&arr1[i1..]);
    }
    arr
}

pub(crate) fn merge_sort<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    if arr.len() < 2 {
        arr.to_vec()
    } else if arr.len() == 2 {
        if arr[0] < arr[1] {
            arr.to_vec()
        } else {
            vec![arr[1], arr[0]]
        }
    } else {
        let m = arr.len() / 2;
        let (arr1, arr2) = (merge_sort(&arr[..m]), merge_sort(&arr[m..]));
        merge(&arr1, &arr2)
    }
}
