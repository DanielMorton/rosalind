pub(crate) fn build_failure_array<N: PartialEq + Copy>(pattern: &[N]) -> Vec<usize> {
    if pattern.is_empty() {
        return vec![];
    }

    let mut lps = vec![0; pattern.len()];
    let mut pos = 0;
    pattern[1..].iter().enumerate().for_each(|(i, &p)| {
        while pos > 0 && pattern[pos] != p {
            pos = lps[pos - 1];
        }

        if pattern[pos] == p {
            pos += 1;
            lps[i + 1] = pos;
        }
    });
    lps
}
