use crate::fasta::Fasta;
use crate::util::{get_prefix, get_suffix, read_string};
use crate::Result;
use std::collections::HashMap;

fn build_overlap_graph(sequences: &[Fasta], k: usize) -> Result<Vec<(String, String)>> {
    let mut edges = Vec::new();

    // For efficiency with larger datasets, we can build a prefix index
    let mut prefix_map: HashMap<String, Vec<&Fasta>> = HashMap::new();

    // Build prefix index
    for seq in sequences {
        if let Some(prefix) = get_prefix(&seq.text, k) {
            prefix_map
                .entry(prefix.to_string())
                .or_insert_with(Vec::new)
                .push(seq);
        }
    }

    // Find overlaps
    for seq1 in sequences {
        if let Some(suffix) = get_suffix(&seq1.text, k) {
            // Look for sequences with matching prefix
            if let Some(matching_seqs) = prefix_map.get(suffix) {
                for &seq2 in matching_seqs {
                    // Ensure we don't create self-loops
                    if seq1.title != seq2.title {
                        edges.push((seq1.title.clone(), seq2.title.clone()));
                    }
                }
            }
        }
    }

    Ok(edges)
}

pub fn execute_grph(input_file: &str) -> Result<()> {
    let data = read_string(input_file)?;
    let dna_list = Fasta::parse(&data)?;
    let overlap_graph = build_overlap_graph(&dna_list, 3)?;
    for (from, to) in overlap_graph {
        println!("{} {}", from, to);
    }
    Ok(())
}
