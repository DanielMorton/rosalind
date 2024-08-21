use std::collections::HashMap;

pub(crate) fn motif_start(dna: &str, motif: &str) -> Vec<usize> {
    let mut starts = Vec::new();
    for i in 0..(dna.len() - motif.len()) {
        if motif == &dna[i..i + motif.len()] {
            starts.push(i + 1)
        }
    }
    starts
}

pub(crate) fn kmer_count(dna: &str, k: usize) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    (0..=dna.len() - k).for_each(|i| {
        *count.entry(dna[i..i+k].to_owned()).or_default() += 1
    });
    count
}