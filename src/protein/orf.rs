use crate::dna::reverse_complement_string;
use crate::protein::codon::load_codon_table;
use crate::util::Result;

fn find_orf(rna: &str) -> Vec<String> {
    let codon_map = load_codon_table();
    let mut orfs = Vec::new();

    // Convert RNA to codons (groups of 3 nucleotides)
    let codons: Vec<String> = rna
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter(|chunk| chunk.len() == 3) // Only complete codons
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    // Find all ORFs starting from each position
    for start_pos in 0..codons.len() {
        // Look for start codon (AUG -> M)
        if let Some(Some('M')) = codon_map.get(&codons[start_pos]) {
            let mut orf = String::new();
            orf.push('M'); // Add the start methionine

            // Translate codons until we hit a stop codon or end of sequence
            for pos in (start_pos + 1)..codons.len() {
                match codon_map.get(&codons[pos]) {
                    Some(Some(amino_acid)) => {
                        orf.push(*amino_acid);
                    }
                    Some(None) => {
                        // Stop codon found - complete ORF
                        orfs.push(orf);
                        break;
                    }
                    None => {
                        // Unknown codon - skip this ORF
                        break;
                    }
                }
            }
        }
    }

    orfs
}
pub(crate) fn find_orfs(dna: &str) -> Result<Vec<String>> {
    let rev_dna = reverse_complement_string(dna)?;
    let rna = dna.replace('T', "U");
    let rev_rna = rev_dna.replace('T', "U");
    let mut orfs = Vec::new();
    for i in 0..=2 {
        orfs.append(&mut find_orf(&rna[i..]));
        orfs.append(&mut find_orf(&rev_rna[i..]));
    }
    Ok(orfs)
}
