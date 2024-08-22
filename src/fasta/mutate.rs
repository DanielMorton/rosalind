use crate::fasta::{dna, FASTA};

fn transition_transversion_count(f1: &dna, f2: &dna) -> (u32, u32) {
    let (mut transitions, mut transversions) = (0, 0);
    f1.chars().zip(f2.chars()).for_each(|(n1, n2)| {
        if (n1 == 'A' && n2 == 'G')
            || (n1 == 'G' && n2 == 'A')
            || (n1 == 'C' && n2 == 'T')
            || (n1 == 'T' && n2 == 'C')
        {
            transitions += 1
        } else if n1 != n2 {
            transversions += 1
        }
    });
    (transitions, transversions)
}

pub(crate) fn transition_transversion_ratio(f1: &dna, f2: &dna) -> f64 {
    let (transitions, tranversions) = transition_transversion_count(f1, f2);
    f64::from(transitions) / f64::from(tranversions)
}
