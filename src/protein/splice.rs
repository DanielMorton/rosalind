use crate::protein::codon::read_codon;

pub(crate) fn rna_splice(dna: &str, introns: &[String]) -> String {
    let codon_map = read_codon();
    let mut rna = dna.replace('T', "U");
    introns.iter().for_each(|intron| {
        rna = rna.replace(&intron.replace('T', "U"), "");
    });
    rna.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .map(|c| codon_map.get(&c).unwrap_or(&" ".to_owned()).to_owned())
        .collect::<String>()
}
