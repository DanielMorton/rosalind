use crate::fasta::FASTA;
use std::fs::read_to_string;

pub(crate) fn read_fasta(file: &str) -> Vec<FASTA> {
    match read_to_string(file).map(|s| {
        s.trim()
            .split("\n>")
            .map(|s| FASTA::read(s))
            .collect::<Vec<_>>()
    }) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    }
}
