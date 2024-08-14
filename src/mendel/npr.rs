use crate::mendel::factorial;
use crate::mendel::ncr::ncr;
use std::collections::HashMap;

pub(crate) fn npr(n: u64, r: u64) -> u64 {
    let mut memo = HashMap::new();
    let c = ncr(n, r, &mut memo) % 1000000;
    (c * (factorial(r as usize) as u64)) % 1000000
}
