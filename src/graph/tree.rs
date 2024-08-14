pub(crate) fn inner_nodes(o: usize) -> usize {
    o - 2
}

pub(crate) fn tree_edge_fill(n: usize, edges: &[(usize, usize)]) -> usize {
    n - edges.len() - 1
}
