use crate::fasta::FASTA;
use crate::graph::graph::reverse_graph;
use crate::graph::make_graph;
use std::collections::{HashMap, HashSet, VecDeque};

pub(crate) fn total_order(graph: &HashMap<FASTA, Vec<FASTA>>) -> Vec<FASTA> {
    let mut reverse = reverse_graph(graph)
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect::<HashSet<_>>()))
        .collect::<HashMap<_, _>>();
    let has_incoming = graph.values().flatten().collect::<HashSet<_>>();
    let mut starting_vertices = graph
        .keys()
        .filter(|&f| !has_incoming.contains(f))
        .map(|f| f.to_owned())
        .collect::<VecDeque<_>>();
    let mut order = Vec::new();
    while !starting_vertices.is_empty() {
        let v = starting_vertices.pop_front().unwrap();
        order.push(v.to_owned());
        graph.get(&v).unwrap().iter().for_each(|n| {
            reverse.entry(n.clone()).and_modify(|prior| {
                let _ = prior.remove(&v);
                if prior.is_empty() {
                    starting_vertices.push_back(n.to_owned());
                }
            });
        });
    }
    order
}

pub(crate) fn align(fasta: &[FASTA]) -> String {
    let graph = make_graph(fasta);
    let order = total_order(&graph);
    let mut alignment = order.first().unwrap().to_owned().dna;
    order[1..].iter().for_each(|f| {
        for k in ((f.len() / 2)..f.len()).rev() {
            if alignment[alignment.len() - k..] == f.dna[..k] {
                alignment += &f.dna[k..].to_string();
                break;
            }
        }
    });
    alignment
}
