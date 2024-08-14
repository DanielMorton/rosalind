

pub(crate) fn dna_prob(dna: &str, p: f64) -> f64 {
    dna.chars()
        .map(|c| if c == 'C' || c == 'G' {f64::log10(p/2.0)} else {f64::log10((1.0 - p)/2.0)}).sum()
}