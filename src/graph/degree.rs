use std::collections::HashMap;
use itertools::Itertools;

pub(crate) fn edges_to_degrees(edges: &[(u32, u32)]) -> Vec<u32> {
    let mut degree_map:HashMap<u32, u32> = HashMap::new();
    for &(v1, v2) in edges {
        *degree_map.entry(v1).or_default() += 1u32;
        *degree_map.entry(v2).or_default() += 1u32;
    }
    degree_map.keys().sorted().map(|k| degree_map.get(k).unwrap().to_owned()).collect::<Vec<_>>()
}