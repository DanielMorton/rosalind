use crate::fasta::FASTA;
use std::cmp::{max, min};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) fn make_graph(fasta: &[FASTA]) -> HashMap<FASTA, Vec<FASTA>> {
    let mut graph = HashMap::new();
    fasta.iter().for_each(|f1| {
        graph.insert(f1.to_owned(), vec![]);
        fasta.iter().for_each(|f2| {
            if f1 != f2 {
                for k in (max(f1.len() / 2, f2.len() / 2)..min(f1.len(), f2.len())).rev() {
                    if f1.text[f1.len() - k..] == f2.text[..k] {
                        match graph.entry(f1.to_owned()) {
                            Entry::Vacant(e) => {
                                e.insert(vec![f2.clone()]);
                            }
                            Entry::Occupied(mut e) => {
                                e.get_mut().push(f2.clone());
                            }
                        };
                        break;
                    }
                }
            }
        })
    });
    graph
}

pub(crate) fn reverse_graph(graph: &HashMap<FASTA, Vec<FASTA>>) -> HashMap<FASTA, Vec<FASTA>> {
    let mut reverse = graph
        .keys()
        .map(|f| (f.clone(), vec![]))
        .collect::<HashMap<_, _>>();
    graph.iter().for_each(|(key, value)| {
        value
            .iter()
            .for_each(|v| match reverse.entry(v.to_owned()) {
                Entry::Vacant(e) => {
                    e.insert(vec![key.clone()]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(key.clone());
                }
            })
    });
    reverse
}
