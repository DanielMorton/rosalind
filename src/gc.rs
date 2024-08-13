use std::fs::read_to_string;

fn gc_content(dna: &str) -> f64 {
    let gc = dna.chars().filter(|&c| c == 'C' || c == 'G').count();
    (gc as f64) / (dna.len() as f64) * 100.0
}

pub(crate) fn gc_max(file: &str) -> (String, f64) {
    let text = match read_to_string(file) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };
    let gc = text
        .split("\n>")
        .map(|s| {
            let mut read = s.split('\n');
            let title = read.next().unwrap().to_owned();
            let dna = read.collect::<String>();
            (title, gc_content(&dna))
        })
        .collect::<Vec<_>>();

    let argmax = gc
        .iter()
        .enumerate()
        .max_by(|(_, (_, g)), (_, (_, g2))| g.partial_cmp(g2).unwrap())
        .map(|(i, _)| i)
        .unwrap();
    (gc[argmax].0.to_owned(), gc[argmax].1)
}
