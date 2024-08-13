pub(crate) fn motif_start(dna: &str, motif: &str) -> Vec<usize> {
    let mut starts = Vec::new();
    for i in 0..(dna.len() - motif.len()) {
        if motif == &dna[i..i + motif.len()] {
            starts.push(i + 1)
        }
    }
    starts
}
