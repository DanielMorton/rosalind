use crate::fasta::Dna;
use crate::util::{read_pair, DNA_PAIRS};

pub(crate) fn reverse_palindrome(dna: &Dna, min_len: usize, max_len: usize) -> Vec<(usize, usize)> {
    let pairs = read_pair(DNA_PAIRS);
    let mut dp = vec![vec![false; dna.len()]; dna.len()];
    for (i, d) in dp.iter_mut().enumerate().take(dna.len()) {
        d[i] = false;
    }
    for (i, d) in dp.iter_mut().enumerate().take(dna.len() - 1)  {
        d[i + 1] =
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
    for (i, d) in dp.iter().enumerate().take(dna.len())  {
        for (j, &di) in d.iter().enumerate().take(dna.len()).skip(i)  {
            if j - i + 1 >= min_len && j - i < max_len && di {
                p.push((i + 1, j - i + 1))
            }
        }
    }
    p
}
