
pub(crate) fn make_dictionary(letters: &[String], n: usize) -> Vec<String> {
    if n == 1 {
        letters.to_vec()
    } else {
        make_dictionary(&letters, n - 1).iter().flat_map(move |d| {
            letters.iter().map(move |l| format!("{}{}", d, l))
        }).collect::<Vec<_>>()
    }
}