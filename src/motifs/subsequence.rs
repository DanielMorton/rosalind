use crate::fasta::FASTA;

pub(crate) fn get_subsequence(text: &FASTA, pattern: &FASTA) -> Vec<usize> {
    let mut t = 0usize;
    let text_chars = text.dna.chars().collect::<Vec<_>>();
    pattern
        .dna
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
