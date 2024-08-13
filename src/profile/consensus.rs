use crate::profile::DNA;

pub(crate) fn find_consensus(dna_list: &[String]) -> String {
    let length = dna_list[0].len();
    let mut profile = vec![vec![0usize; 4]; length];
    dna_list.iter().for_each(|dna| {
        dna.chars().enumerate().for_each(|(i, n)| {
            if n == 'A' {
                profile[i][0] += 1
            } else if n == 'C' {
                profile[i][1] += 1
            } else if n == 'G' {
                profile[i][2] += 1
            } else if n == 'T' {
                profile[i][3] += 1
            }
        })
    });
    profile
        .iter()
        .map(|p| {
            p.iter()
                .enumerate()
                .max_by_key(|(_, &n)| n)
                .map(|(i, _)| i)
                .map(|i| DNA[i])
                .unwrap()
                .to_owned()
        })
        .collect::<String>()
}
