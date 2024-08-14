pub(crate) fn factorial(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        n * factorial(n - 1)
    }
}
pub(crate) fn permute(n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        vec![vec![1]]
    } else {
        permute(n - 1)
            .into_iter()
            .map(|v| (0..=n - 1).map(move |i| vec![&v[..i], &vec![n][..], &v[i..]].concat()))
            .flatten()
            .collect::<Vec<_>>()
    }
}
