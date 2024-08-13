use crate::protein::CODONS;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

pub(crate) fn read_codon() -> HashMap<String, String> {
    let codons = match fs::read_to_string(CODONS) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };
    codons
        .split('\n')
        .map(|codon| codon.split(' ').collect::<Vec<_>>())
        .filter(|codon| codon.len() == 2)
        .map(|codon| (codon[0].to_owned(), codon[1].to_owned()))
        .collect::<HashMap<_, _>>()
}

pub(crate) fn reverse_codon(codon: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut reverse = HashMap::new();
    codon
        .iter()
        .for_each(|(r, p)| match reverse.entry(p.clone()) {
            Entry::Vacant(e) => {
                e.insert(vec![r.clone()]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(r.clone());
            }
        });
    reverse
}

pub(crate) fn reverse_count(reverse: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    reverse
        .iter()
        .map(|(k, v)| (k.clone(), v.len()))
        .collect::<HashMap<_, _>>()
}
