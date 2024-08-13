pub(crate) fn nucleotide_count(dna: &str) -> Vec<usize> {
    let mut count = vec![0; 4];
    for n in dna.chars() {
        if n == 'A' {
            count[0] += 1
        } else if n == 'C' {
            count[1] += 1
        } else if n == 'G' {
            count[2] += 1
        } else if n == 'T' {
            count[3] += 1
        }
    }
    count
}

pub(crate) fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

fn complement(n: char) -> char {
    if n == 'A' {
        'T'
    } else if n == 'C' {
        'G'
    } else if n == 'G' {
        'C'
    } else if n == 'T' {
        'A'
    } else {
        panic!("Invalid character {}", n)
    }
}

pub(crate) fn reverse_complement(dna: &str) -> String {
    dna.trim()
        .chars()
        .rev()
        .collect::<String>()
        .chars()
        .map(complement)
        .collect::<String>()
}
