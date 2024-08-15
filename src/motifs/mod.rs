mod file;
mod find;
mod hamming;
mod lcs;
mod motif;
mod palindrome;

static DNA: &[&str] = &["A", "C", "G", "T"];

static PAIRS: &str = "/Users/dmorton/IdeaProjects/rosalind/dna_pair.txt";

pub(super) use file::read_two_line;
pub(super) use find::find_motifs;
pub(super) use hamming::hamming_distance;
pub(super) use lcs::lcs;
pub(super) use motif::motif_start;
pub(super) use palindrome::reverse_palindrome;
