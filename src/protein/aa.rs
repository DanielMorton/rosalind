use crate::protein::codon::load_codon_table;
use crate::util::Result;
use std::collections::HashMap;

pub fn rna_to_protein(rna: &str) -> Result<String> {
    let codon_table = load_codon_table();
    let mut protein = String::new();

    // Process RNA in chunks of 3 (codons)
    for chunk in rna.chars().collect::<Vec<char>>().chunks(3) {
        if chunk.len() == 3 {
            let codon: String = chunk.iter().collect();

            if let Some(amino_acid_option) = codon_table.get(&codon) {
                match amino_acid_option {
                    Some(amino_acid) => protein.push(*amino_acid),
                    None => break, // Stop codon encountered
                }
            } else {
                eprintln!("Warning: Unknown codon '{}'", codon);
            }
        }
    }

    Ok(protein)
}

pub fn rna_count(protein: &str) -> usize {
    let codon_map = load_codon_table();
    let mut amino_acid_counts = HashMap::new();

    // Count codons for each amino acid
    for (_, amino_acid_opt) in codon_map {
        if let Some(amino_acid) = amino_acid_opt {
            *amino_acid_counts.entry(amino_acid).or_insert(0) += 1;
        }
    }

    let mut result = 3; // Factor of 3 for stop codons
    for c in protein.trim().chars() {
        let count = *amino_acid_counts.get(&c).unwrap_or(&1);
        result = (result * count) % 1000000;
    }
    result
}
