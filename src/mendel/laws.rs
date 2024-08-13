use crate::mendel::ncr::ncr;
use std::collections::HashMap;

pub(crate) fn first_law(k: u32, m: u32, n: u32) -> f64 {
    let (kf, mf, nf) = (f64::from(k), f64::from(m), f64::from(n));
    let num = kf * (kf - 1.0) / 2.0
        + kf * mf
        + kf * nf
        + mf * (mf - 1.0) / 2.0 * 3.0 / 4.0
        + mf * nf / 2.0;
    let denom = (kf + mf + nf) * (kf + mf + nf - 1.0) / 2.0;
    num / denom
}

pub(crate) fn expected_offspring(nums: &[u32]) -> f64 {
    2.0 * f64::from(nums[0])
        + 2.0 * f64::from(nums[1])
        + 2.0 * f64::from(nums[2])
        + 1.5 * f64::from(nums[3])
        + 1.0 * f64::from(nums[4])
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
