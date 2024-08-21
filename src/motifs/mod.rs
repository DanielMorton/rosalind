mod dictionary;
mod find;
mod hamming;
mod kmp;
mod lcs;
mod motif;
mod palindrome;
pub mod subsequence;

pub(super) use dictionary::make_dictionary;
pub(super) use find::find_motifs;
pub(super) use hamming::hamming_distance;
pub(super) use kmp::build_failure_array;
pub(super) use lcs::lcs;
pub(super) use motif::{kmer_count, motif_start};
pub(super) use palindrome::reverse_palindrome;
pub(super) use subsequence::get_subsequence;
