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
            .flat_map(|v| (0..=n - 1).map(move |i| [&v[..i], &vec![n][..], &v[i..]].concat()))
            .collect::<Vec<_>>()
    }
}
