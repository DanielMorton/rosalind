use crate::protein::codon::load_codon_table;

pub(crate) fn rna_splice(dna: &str, introns: &[String]) -> String {
    let codon_map = load_codon_table();
    let mut rna = dna.replace('T', "U");
    introns.iter().for_each(|intron| {
        rna = rna.replace(&intron.replace('T', "U"), "");
    });
    rna.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter(|chunk| chunk.len() == 3)
        .map(|c| c.iter().collect::<String>())
        .filter_map(|c| codon_map.get(&c).and_then(|aa| *aa))
        .collect::<String>()
}
