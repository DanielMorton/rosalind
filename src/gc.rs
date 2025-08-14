use crate::fasta::Fasta;
use crate::util::{read_string, Error};
use crate::util::Result;

/// Calculate GC content for a raw DNA string
pub fn gc_content(dna: &str) -> f64 {
    if dna.is_empty() {
        return 0.0;
    }

    let gc_count = dna.chars().filter(|&c| matches!(c, 'G' | 'C')).count();

    (gc_count as f64 / dna.len() as f64) * 100.0
}

pub fn find_highest(sequences: &[Fasta]) -> Option<&Fasta> {
    sequences.iter().max_by(|&a, &b| {
        a.gc_content()
            .partial_cmp(&b.gc_content())
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

pub fn execute_gc(input_file: &str) -> Result<()> {
    let content = read_string(input_file)?;
    let sequences = Fasta::parse(&content)?;

    if sequences.is_empty() {
        return Err(Error::Parse("No FASTA sequences found".to_string()));
    }

    let highest = find_highest(&sequences)
        .ok_or_else(|| Error::Parse("Unable to determine highest GC content".to_string()))?;

    println!("{}", highest.title);
    println!("{:.6}", highest.gc_content());

    Ok(())
}
