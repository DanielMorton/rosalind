use crate::protein::{rna_to_protein, CODONS};
use crate::util::{read_string, Result};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

pub fn load_codon_table() -> HashMap<String, Option<char>> {
    let mut table = HashMap::new();

    let content = fs::read_to_string(CODONS).expect("Failed to read codon table file");

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 1 {
            // Stop codon (no amino acid)
            table.insert(parts[0].to_string(), None);
        } else if parts.len() == 2 {
            // Regular codon with amino acid
            let codon = parts[0].to_string();
            let amino_acid = parts[1].chars().next().unwrap();
            table.insert(codon, Some(amino_acid));
        }
    }

    table
}

pub(crate) fn reverse_codon(codon: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut reverse = HashMap::new();
    codon
        .iter()
        .for_each(|(r, p)| match reverse.entry(p.to_owned()) {
            Entry::Vacant(e) => {
                e.insert(vec![r.clone()]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(r.clone());
            }
        });
    reverse
}

pub(crate) fn reverse_count(reverse: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    reverse
        .iter()
        .map(|(k, v)| (k.clone(), v.len()))
        .collect::<HashMap<_, _>>()
}

pub(crate) fn execute_prot(input_file: &str) -> Result<()> {
    let rna = read_string(input_file)?;
    let protein = rna_to_protein(&rna)?;
    println!("{protein}");
    Ok(())
}
