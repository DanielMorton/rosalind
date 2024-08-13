mod aa;
mod codon;
mod mass;
mod orf;

pub(super) use aa::{rna_count, rna_to_protein};
pub(super) use mass::protein_mass;
pub(super) use orf::find_orfs;

static CODONS: &str = "/Users/dmorton/IdeaProjects/rosalind/codon.txt";
static MASS: &str = "/Users/dmorton/IdeaProjects/rosalind/aa_mass.txt";
