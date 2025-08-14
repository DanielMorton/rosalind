use crate::fasta::Fasta;
use crate::Result;

pub(crate) fn pairs(file: &str, k: usize) -> Result<Vec<(String, String)>> {
    let fasta = Fasta::parse(file)?;
    let mut pairs = Vec::new();
    fasta.iter().enumerate().for_each(|(i, f1)| {
        fasta[..i].iter().for_each(|f2| {
            if f1.text != f2.text {
                if f1.text[..k] == f2.text[f2.text.len() - k..] {
                    pairs.push((f2.title.to_owned(), f1.title.to_owned()));
                }
                if f2.text[..k] == f1.text[f1.text.len() - k..] {
                    pairs.push((f1.title.to_owned(), f2.title.to_owned()))
                }
            }
        })
    });
    Ok(pairs)
}
