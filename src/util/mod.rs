mod pairs;
mod string;
mod search;

pub(super) use pairs::read_pair;
pub(super) use search::binary_search;
pub(super) use string::{read_num_list, read_string, read_two_line, read_vec};

pub(super) static DNA: &[&str] = &["A", "C", "G", "T"];
pub(super) static DNA_PAIRS: &str = "/Users/dmorton/IdeaProjects/rosalind/dna_pair.txt";

pub(super) static RNA: &[&str] = &["A", "C", "G", "U"];

pub(super) static RNA_PAIRS: &str = "/Users/dmorton/IdeaProjects/rosalind/rna_pair.txt";
