use crate::util::{read_two_line, Result};
use std::collections::HashMap;

// KMP (Knuth-Morris-Pratt) Algorithm - O(n + m) time complexity
fn compute_lps(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    lps
}

fn find_substring_locations_kmp(s: &str, t: &str) -> Vec<usize> {
    let mut locations = Vec::new();

    if t.is_empty() || t.len() > s.len() {
        return locations;
    }

    let text = s.as_bytes();
    let pattern = t.as_bytes();
    let n = text.len();
    let m = pattern.len();

    let lps = compute_lps(pattern);

    let mut i = 0; // index for text
    let mut j = 0; // index for pattern

    while i < n {
        if pattern[j] == text[i] {
            i += 1;
            j += 1;
        }

        if j == m {
            locations.push(i - j + 1); // Convert to 1-based indexing
            j = lps[j - 1];
        } else if i < n && pattern[j] != text[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    locations
}

// Alternative: Boyer-Moore-Horspool (simpler, often faster in practice for short patterns)
fn find_substring_locations_bmh(s: &str, t: &str) -> Vec<usize> {
    let mut locations = Vec::new();

    if t.is_empty() || t.len() > s.len() {
        return locations;
    }

    let text = s.as_bytes();
    let pattern = t.as_bytes();
    let n = text.len();
    let m = pattern.len();

    // Build bad character table
    let mut bad_char = [m; 256];
    for i in 0..m {
        bad_char[pattern[i] as usize] = m - 1 - i;
    }

    let mut i = 0;
    while i <= n - m {
        let mut j = m - 1;

        // Check pattern from right to left
        while j < m && pattern[j] == text[i + j] {
            if j == 0 {
                locations.push(i + 1); // Convert to 1-based indexing
                break;
            }
            j -= 1;
        }

        // Calculate skip distance
        if j < m {
            let skip = if i + m < n {
                bad_char[text[i + m] as usize]
            } else {
                1
            };
            i += skip.max(1);
        } else {
            i += 1;
        }
    }

    locations
}

pub(crate) fn kmer_count(dna: &str, k: usize) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    (0..=dna.len() - k).for_each(|i| *count.entry(dna[i..i + k].to_owned()).or_default() += 1);
    count
}

pub fn execute_subs(input_file: &str) -> Result<()> {
    let (dna, motif) = read_two_line(input_file)?;
    find_substring_locations_kmp(&dna, &motif)
        .iter()
        .for_each(|p| print!("{p} "));
    println!();
    Ok(())
}
