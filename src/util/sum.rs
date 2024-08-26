use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) fn two_sum(arr: &[i32]) -> Vec<i32> {
    let mut elements = HashMap::new();
    let mut pair = vec![-1];
    arr.iter()
        .enumerate()
        .for_each(|(i, &a)| match elements.entry(-a) {
            Entry::Vacant(_) => {
                elements.insert(a, i);
            }
            Entry::Occupied(e) => {
                pair = vec![(e.get() + 1) as i32, (i + 1) as i32];
            }
        });
    pair
}
