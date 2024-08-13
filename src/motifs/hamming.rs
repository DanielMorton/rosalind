pub(crate) fn hamming_distance(s: &str, t: &str) -> usize {
    s.chars().zip(t.chars()).filter(|(c1, c2)| c1 != c2).count()
}
