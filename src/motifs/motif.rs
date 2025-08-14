use crate::util::{read_two_line, Result};
use std::collections::HashMap;

pub fn find_substring_locations(s: &str, t: &str) -> Vec<usize> {
    if t.is_empty() {
        return Vec::new();
    }

    s.match_indices(t)
        .map(|(index, _)| index + 1) // Convert to 1-indexed
        .collect()
}

pub(crate) fn kmer_count(dna: &str, k: usize) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    (0..=dna.len() - k).for_each(|i| *count.entry(dna[i..i + k].to_owned()).or_default() += 1);
    count
}

pub fn execute_subs(input_file: &str) -> Result<()> {
    let (dna, motif) = read_two_line(input_file)?;
    find_substring_locations(&dna, &motif)
        .iter()
        .for_each(|p| print!("{p} "));
    println!();
    Ok(())
}
