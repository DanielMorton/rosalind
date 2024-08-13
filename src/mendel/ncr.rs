use std::collections::HashMap;

pub(crate) fn ncr(n: u64, r: u64, ncr_map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if !ncr_map.contains_key(&(n, r)) {
        if r == 0 {
            ncr_map.insert((n, r), 1);
        } else if r == 1 {
            ncr_map.insert((n, r), n);
        } else if r > n - r {
            let c = ncr(n, n - r, ncr_map);
            ncr_map.insert((n, r), c);
        } else {
            let mut c = ncr(n - 1, r - 1, ncr_map);
            c += ncr(n - 1, r, ncr_map);
            ncr_map.insert((n, r), c);
        }
    }
    *ncr_map.get(&(n, r)).unwrap()
}
