use crate::fasta::FASTA;
use crate::motifs::DNA;
use generalized_suffix_tree::GeneralizedSuffixTree;

pub(crate) fn lcs(fasta: &[FASTA]) -> String {
    let mut tree = GeneralizedSuffixTree::new();
    let mut i = 0;
    fasta.iter().for_each(|f| {
        if DNA.contains(&&*char::from_u32(i).unwrap().to_string()) {
            i += 1;
        }
        tree.add_string(f.dna.clone(), char::from_u32(i as u32).unwrap());
        i += 1
    });
    tree.longest_common_substring_all()
}
