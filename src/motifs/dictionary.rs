pub(crate) fn make_dictionary(letters: &[&str], n: usize) -> Vec<String> {
    if n == 1 {
        letters.iter().map(|l| l.to_string()).collect::<Vec<_>>()
    } else {
        make_dictionary(&letters, n - 1)
            .iter()
            .flat_map(move |d| letters.iter().map(move |l| format!("{}{}", d, l)))
            .collect::<Vec<_>>()
    }
}
