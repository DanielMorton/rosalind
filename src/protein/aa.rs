use crate::protein::codon::{read_codon, reverse_codon, reverse_count};

pub fn rna_to_protein(rna: &str) -> String {
    let codon_map = read_codon();
    rna.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .map(|c| codon_map.get(&c).unwrap_or(&"".to_owned()).to_owned())
        .collect::<String>()
}

pub fn rna_count(protein: &str) -> usize {
    let codon_map = read_codon();
    let reverse = reverse_codon(&codon_map);
    let codon_count = reverse_count(&reverse);
    3 * protein
        .trim()
        .chars()
        .map(|c| *codon_count.get(&c.to_string()).unwrap_or(&1))
        .reduce(|mut x, y| {
            x *= y;
            x %= 1000000;
            x
        })
        .unwrap()
        % 1000000
}
