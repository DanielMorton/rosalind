use crate::dna::reverse_complement;
use crate::protein::codon::read_codon;

fn find_orf(rna: &str) -> Vec<String> {
    let codon_map = read_codon();
    let aa_list = rna
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .map(|c| codon_map.get(&c).unwrap_or(&" ".to_owned()).to_owned())
        .collect::<String>();
    let (mut left, mut right) = (0, 0);
    let mut orfs = Vec::new();
    while right < aa_list.len() {
        if aa_list.chars().nth(right).unwrap() != ' ' || left == right {
            right += 1;
        } else if aa_list.chars().nth(left).unwrap() != 'M' && left <= right {
            left += 1;
        } else {
            orfs.push(aa_list[left..right].to_owned());
            left += 1
        }
    }
    orfs
}

pub(crate) fn find_orfs(dna: &str) -> Vec<String> {
    let rev_dna = reverse_complement(dna);
    let rna = dna.replace('T', "U");
    let rev_rna = rev_dna.replace('T', "U");
    let mut orfs = Vec::new();
    for i in 0..=2 {
        orfs.append(&mut find_orf(&rna[i..]));
        orfs.append(&mut find_orf(&rev_rna[i..]));
    }
    orfs
}
