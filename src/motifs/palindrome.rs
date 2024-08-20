use crate::util::{read_pair, DNA_PAIRS};

pub(crate) fn reverse_palindrome(dna: &str, min_len: usize, max_len: usize) -> Vec<(usize, usize)> {
    let pairs = read_pair(&DNA_PAIRS);
    let mut dp = vec![vec![false; dna.len()]; dna.len()];
    for i in 0..dna.len() {
        dp[i][i] = false;
    }
    for i in 0..(dna.len() - 1) {
        dp[i][i + 1] =
            pairs.get(&dna.chars().nth(i).unwrap()).unwrap() == &dna.chars().nth(i + 1).unwrap();
    }
    for l in 3..dna.len() {
        let (mut i, mut j) = (0, l - 1);
        while j < dna.len() {
            dp[i][j] = dp[i + 1][j - 1]
                && pairs.get(&dna.chars().nth(i).unwrap()).unwrap() == &dna.chars().nth(j).unwrap();
            i += 1;
            j += 1
        }
    }
    let mut p = Vec::new();
    for i in 0..dna.len() {
        for j in i..dna.len() {
            if j - i + 1 >= min_len && j - i + 1 <= max_len && dp[i][j] {
                p.push((i + 1, j - i + 1))
            }
        }
    }
    p
}
