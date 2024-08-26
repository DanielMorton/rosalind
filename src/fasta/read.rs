use crate::fasta::Fasta;
use std::fs::read_to_string;

pub(crate) fn read_fasta(file: &str) -> Vec<Fasta> {
    match read_to_string(file).map(|s| {
        s.trim()
            .split("\n>")
            .map(Fasta::read)
            .collect::<Vec<_>>()
    }) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    }
}
