use crate::dna::transcribe_string;
use crate::fasta::Fasta;
use crate::protein::codon::load_codon_table;
use crate::protein::rna_to_protein;
use crate::util::{read_string, Error};
use crate::Result;
use std::collections::HashMap;

fn remove_introns(dna: &str, introns: &[String]) -> Result<String> {
    let mut result = dna.to_string();

    // Remove each intron from the DNA sequence
    // We need to be careful about the order - longer introns should be removed first
    // to avoid partial matches
    let mut sorted_introns = introns.to_vec();
    sorted_introns.sort_by_key(|intron| std::cmp::Reverse(intron.len()));

    for intron in &sorted_introns {
        result = result.replace(intron, "");
    }

    Ok(result)
}

fn solve_rna_splicing(
    fasta: &[Fasta],
    codon_map: &HashMap<String, Option<char>>,
) -> Result<String> {
    if fasta.is_empty() {
        return Err(Error::EmptyInput);
    }

    // First sequence is the main DNA string
    let dna_string = &fasta[0].text;

    // Remaining sequences are introns
    let introns: Vec<String> = fasta[1..].iter().map(|seq| seq.text.clone()).collect();

    // Remove introns to get exons
    let exons = remove_introns(dna_string, &introns)?;

    // Transcribe DNA to RNA
    let rna = transcribe_string(&exons)?;

    // Translate RNA to protein
    let protein = rna_to_protein(&rna, &codon_map)?;

    Ok(protein)
}

pub fn execute_splc(input_file: &str) -> Result<()> {
    let text = read_string(input_file)?;
    let fasta = Fasta::parse(&text)?;
    let codon_map = load_codon_table();
    let protein = solve_rna_splicing(&fasta, &codon_map)?;
    println!("{}", protein);
    Ok(())
}
