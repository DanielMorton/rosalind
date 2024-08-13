pub(crate) fn k_fibonacci(n: u64, k: u64) -> u64 {
    let (mut current, mut previous) = (1, 1);
    let mut iter = 2;
    while iter < n {
        (previous, current) = (current, current + k * previous);
        iter += 1
    }
    current
}
