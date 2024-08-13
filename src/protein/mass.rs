use crate::protein::MASS;
use std::collections::HashMap;
use std::fs;

pub(crate) fn aa_mass() -> HashMap<String, f64> {
    let masses = match fs::read_to_string(MASS) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };
    masses
        .split('\n')
        .map(|m| {
            let mut split = m.split(' ');
            (
                split.next().unwrap().to_owned(),
                split.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>()
}

pub(crate) fn protein_mass(protein: &str) -> f64 {
    let masses = aa_mass();
    println!("{:?}", masses);
    protein
        .chars()
        .map(|c| masses.get(&c.to_string()).unwrap())
        .sum()
}
