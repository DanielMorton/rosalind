use std::collections::HashMap;
use std::fs;

pub(crate) fn read_pair(file: &str) -> HashMap<char, char> {
    let codons = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };
    codons
        .split('\n')
        .map(|pair| pair.split(' ').collect::<Vec<_>>())
        .map(|pair| {
            (
                pair[0].chars().next().unwrap(),
                pair[1].chars().next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>()
}
