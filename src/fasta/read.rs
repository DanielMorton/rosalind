use std::fs::read_to_string;
use crate::fasta::FASTA;

pub(crate) fn read_fasta(file: &str) -> Vec<FASTA> {
    match read_to_string(file).map(|s| {
        s.trim()
            .split("\n>")
            .map(|s| {
                let mut read = s.split('\n');
                let title = read.next().unwrap().replace('>', "");
                let dna = read.collect::<String>();
                FASTA::new(&title, &dna)
            })
            .collect::<Vec<_>>()
    }) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    }
}
