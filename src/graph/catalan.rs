use crate::fasta::rna;
use crate::util::{read_pair, RNA_PAIRS};
use std::collections::HashMap;

fn catalan_dp(
    rna: &[char],
    left: usize,
    right: usize,
    dp: &mut HashMap<(usize, usize), u64>,
    rna_pair: &HashMap<char, char>,
) -> u64 {
    if (right - left) % 2 == 0 {
        0
    } else if left >= right || left >= rna.len() || right + 1 == 0 {
        1
    } else if dp.contains_key(&(left, right)) {
        dp.get(&(left, right)).unwrap().to_owned()
    } else {
        let letter = rna[left];
        let letter_match = rna_pair.get(&letter).unwrap().to_owned();
        let tot = (left + 1..=right + 1)
            .step_by(2)
            .filter(|&i| rna[i] == letter_match)
            .map(|i| {
                (catalan_dp(rna, left + 1, i - 1, dp, &rna_pair)
                    * catalan_dp(rna, i + 1, right, dp, &rna_pair))
                    % 1000000
            })
            .collect::<Vec<_>>()
            .iter()
            .sum::<u64>()
            % 1000000;
        dp.insert((left, right), tot);
        tot
    }
}

pub(crate) fn catalan_number(rna: &rna) -> u64 {
    let rna_pair = read_pair(&RNA_PAIRS);
    let rna_vec = rna.chars().collect::<Vec<_>>();
    let mut dp = HashMap::new();
    catalan_dp(&rna_vec, 0, rna.len() - 1, &mut dp, &rna_pair)
}
