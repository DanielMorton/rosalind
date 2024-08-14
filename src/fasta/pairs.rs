use crate::fasta::read::read_fasta;

pub(crate) fn pairs(file: &str, k: usize) -> Vec<(String, String)> {
    let fasta = read_fasta(file);
    let mut pairs = Vec::new();
    fasta.iter().enumerate().for_each(|(i, f1)| {
        fasta[..i].iter().for_each(|f2| {
            if f1.dna != f2.dna {
                if f1.dna[..k] == f2.dna[f2.dna.len() - k..] {
                    pairs.push((f2.title.to_owned(), f1.title.to_owned()));
                }
                if f2.dna[..k] == f1.dna[f1.dna.len() - k..] {
                    pairs.push((f1.title.to_owned(), f2.title.to_owned()))
                }
            }
        })
    });
    pairs
}
