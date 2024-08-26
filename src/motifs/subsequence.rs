use crate::fasta::Fasta;

pub(crate) fn get_subsequence(text: &Fasta, pattern: &Fasta) -> Vec<usize> {
    let mut t = 0usize;
    let text_chars = text.chars().collect::<Vec<_>>();
    pattern
        .chars()
        .map(|n| {
            while n != text_chars[t] {
                t += 1;
            }
            t += 1;
            t
        })
        .collect::<Vec<_>>()
}
