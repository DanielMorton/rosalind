use crate::util::{read_string, Error, Result};

#[derive(Debug, Clone)]
struct DnaNucleotideCounts {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}
impl DnaNucleotideCounts {
    fn new() -> Self {
        Self { a: 0, c: 0, g: 0, t: 0 }
    }

    fn count_nucleotides_in_string(dna_string: &str) -> Self {
        let mut counts = DnaNucleotideCounts::new();

        for nucleotide in dna_string.chars() {
            match nucleotide {
                'A' => counts.a += 1,
                'C' => counts.c += 1,
                'G' => counts.g += 1,
                'T' => counts.t += 1,
                _ => {} // Ignore any other characters (like newlines)
            }
        }

        counts
    }
}

pub fn execute_dna(input_file: &str) -> Result<()> {
    let dna = read_string(input_file)?;

    if dna.is_empty() {
        return Err(Error::EmptyInput);
    }

    let counts = DnaNucleotideCounts::count_nucleotides_in_string(&dna);
    println!("{} {} {} {}", counts.a, counts.c, counts.g, counts.t);

    Ok(())
}

pub(crate) fn rna_nucleotide_count(dna: &str) -> Vec<usize> {
    let mut count = vec![0; 4];
    for n in dna.chars() {
        if n == 'A' {
            count[0] += 1
        } else if n == 'C' {
            count[1] += 1
        } else if n == 'G' {
            count[2] += 1
        } else if n == 'U' {
            count[3] += 1
        }
    }
    count
}

fn transcribe_string(dna_string: &str) -> String {
    dna_string.replace('T', "U")
}
pub fn transcribe_dna_to_rna(input_file: &str) -> Result<()> {
    let dna_string = read_string(input_file)?;
    let rna_string = transcribe_string(&dna_string);

    // Print the result
    println!("{}", rna_string);

    Ok(())
}

pub fn reverse_complement_string(dna_string: &str) -> Result<String> {
    Ok(dna_string
        .chars()
        .rev()
        .map(|nucleotide| match nucleotide {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            other => other,
        })
        .collect())
}
pub fn reverse_complement(input_file: &str) -> Result<()> {
    let dna_string = read_string(input_file)?;
    let rev_comp = reverse_complement_string(&dna_string)?;

    println!("{}", rev_comp);

    Ok(())
}
