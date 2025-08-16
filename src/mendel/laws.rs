use crate::mendel::ncr::ncr;
use crate::util::{read_num_list, read_string, Error, Result};
use std::collections::HashMap;

fn mendel_first_law(k: u64, m: u64, n: u64) -> f64 {
    let k = k as f64;
    let m = m as f64;
    let n = n as f64;
    let total = k + m + n;
    let denom = total * (total - 1.0); // ordered pairs

    // Probability of recessive phenotype
    let p_rr_rr = (n * (n - 1.0)) / denom * 1.0; // rr x rr
    let p_rr_Rr = (n * m) / denom * 0.5 * 2.0; // rr x Rr and Rr x rr
    let p_Rr_Rr = (m * (m - 1.0)) / denom * 0.25; // Rr x Rr

    let p_recessive = p_rr_rr + p_rr_Rr + p_Rr_Rr;
    1.0 - p_recessive
}

pub(crate) fn execute_iprb(input_file: &str) -> Result<()> {
    let text = read_string(input_file)?;
    let mut nums = text.split_whitespace();

    let k: u64 = nums
        .next()
        .ok_or_else(|| Error::Parse("Missing k".to_string()))?
        .parse()
        .map_err(|_| Error::Parse("Invalid k".to_string()))?;
    let m: u64 = nums
        .next()
        .ok_or_else(|| Error::Parse("Missing m".to_string()))?
        .parse()
        .map_err(|_| Error::Parse("Invalid m".to_string()))?;
    let n: u64 = nums
        .next()
        .ok_or_else(|| Error::Parse("Missing n".to_string()))?
        .parse()
        .map_err(|_| Error::Parse("Invalid n".to_string()))?;

    println!("{:.5}", mendel_first_law(k, m, n));
    Ok(())
}

pub fn expected_dominant_offspring(counts: &[u32]) -> Result<f64> {
    if counts.len() != 6 {
        return Err(Error::Parse("Expected exactly 6 genotype counts".into()));
    }

    // Probabilities of dominant phenotype for each cross type
    let probabilities = [1.0, 1.0, 1.0, 0.75, 0.5, 0.0];

    let expected: f64 = counts
        .iter()
        .zip(probabilities.iter())
        .map(|(&count, &prob)| count as f64 * 2.0 * prob)
        .sum();

    Ok(expected)
}

pub fn execute_iev(input_file: &str) -> Result<()> {
    let counts = read_num_list::<u32>(input_file, ' ')?;
    let expected = expected_dominant_offspring(&counts)?;
    println!("{:.5}", expected);
    Ok(())
}

pub(crate) fn second_law(k: u32, n: u32, p: f64) -> f64 {
    let mut ncr_map: HashMap<(u64, u64), u64> = HashMap::new();
    let max = 2u32.pow(k);
    (n..=max)
        .map(|r| {
            let c = ncr(u64::from(max), u64::from(r), &mut ncr_map);
            (c as f64) * p.powf(f64::from(r)) * (1.0 - p).powf(f64::from(max - r))
        })
        .sum::<f64>()
}
