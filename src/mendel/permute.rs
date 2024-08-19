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

pub(crate) fn permutation_list(n: i32) -> Vec<Vec<i32>> {
    if n == 1 {
        vec![vec![1], vec![-1]]
    } else {
        permutation_list(n - 1)
            .into_iter()
            .flat_map(|v| {
                (0..=n - 1)
                    .map(|i| i as usize)
                    .map(move |i| {
                        vec![
                            [&v[..i], &vec![n][..], &v[i..]].concat(),
                            [&v[..i], &vec![-n][..], &v[i..]].concat(),
                        ]
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}
